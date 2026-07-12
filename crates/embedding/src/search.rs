//! Code search over HNSW + optional semantic fuse.

use std::sync::Arc;

use telepathic_semantic::{SemanticConfig, SemanticFunc, combined_score};

use crate::document::{CodeDocument, format_query};
use crate::engine::EmbeddingEngine;
use crate::error::{EmbeddingError, EmbeddingResult};
use crate::index::HnswIndex;

/// Neural weight in fused score (`0.7 * neural + 0.3 * semantic`).
pub const NEURAL_WEIGHT: f32 = 0.7;
/// Semantic weight in fused score.
pub const SEMANTIC_WEIGHT: f32 = 0.3;

/// Fuse neural cosine with algorithmic semantic score.
pub fn fuse_scores(neural: f32, semantic: f32) -> f32 {
  NEURAL_WEIGHT * neural + SEMANTIC_WEIGHT * semantic
}

/// Optional semantic boost keyed by document / qualified name.
pub trait SemanticBoost: Send + Sync {
  /// Return a `[0, 1]`-ish score for `doc_id` (or `None` to skip).
  fn score(&self, doc_id: &str) -> Option<f32>;
}

/// Lookup table boost.
pub struct MapBoost(pub rustc_hash::FxHashMap<String, f32>);

impl SemanticBoost for MapBoost {
  fn score(&self, doc_id: &str) -> Option<f32> {
    self.0.get(doc_id).copied().or_else(|| {
      // Allow boosting by qualified_name without chunk suffix.
      doc_id
        .split_once('#')
        .and_then(|(qn, _)| self.0.get(qn).copied())
    })
  }
}

/// Pair of [`SemanticFunc`] scorers using [`combined_score`].
pub struct CombinedSemanticBoost<'a> {
  pub query: SemanticFunc<'a>,
  pub by_qn: rustc_hash::FxHashMap<String, SemanticFunc<'a>>,
  pub config: SemanticConfig,
}

impl SemanticBoost for CombinedSemanticBoost<'_> {
  fn score(&self, doc_id: &str) -> Option<f32> {
    let qn = doc_id.split_once('#').map(|(q, _)| q).unwrap_or(doc_id);
    let other = self.by_qn.get(qn)?;
    Some(combined_score(&self.query, other, &self.config))
  }
}

/// One search hit.
#[derive(Debug, Clone)]
pub struct SearchHit {
  pub document: CodeDocument,
  pub neural_score: f32,
  pub semantic_score: Option<f32>,
  pub score: f32,
}

/// Indexes code documents and runs neural (+ fused) search.
pub struct CodeSearcher {
  engine: Arc<dyn EmbeddingEngine>,
  index: HnswIndex,
}

impl CodeSearcher {
  pub fn new(engine: Arc<dyn EmbeddingEngine>) -> EmbeddingResult<Self> {
    let index = HnswIndex::new(engine.dimension())?;
    Ok(Self { engine, index })
  }

  pub fn engine(&self) -> &Arc<dyn EmbeddingEngine> {
    &self.engine
  }

  pub fn len(&self) -> usize {
    self.index.len()
  }

  pub fn is_empty(&self) -> bool {
    self.index.is_empty()
  }

  /// Embed and upsert documents into the HNSW index.
  pub async fn index_documents(&mut self, docs: &[CodeDocument]) -> EmbeddingResult<()> {
    if docs.is_empty() {
      return Ok(());
    }
    let texts: Vec<&str> = docs.iter().map(|d| d.text.as_str()).collect();
    let batch = self.engine.batch_size().max(1);
    let mut offset = 0;
    while offset < texts.len() {
      let end = (offset + batch).min(texts.len());
      let embeddings = self.engine.embed(&texts[offset..end]).await?;
      if embeddings.len() != end - offset {
        return Err(EmbeddingError::Inference(format!(
          "expected {} embeddings, got {}",
          end - offset,
          embeddings.len()
        )));
      }
      for (doc, vec) in docs[offset..end].iter().zip(embeddings) {
        self.index.upsert(doc.clone(), &vec)?;
      }
      offset = end;
    }
    Ok(())
  }

  /// Neural-only search.
  ///
  /// `natural_language = true` applies CodeRankEmbed query prefix.
  pub async fn search(
    &self,
    query: &str,
    k: usize,
    natural_language: bool,
  ) -> EmbeddingResult<Vec<SearchHit>> {
    let formatted = format_query(query, natural_language);
    if formatted.is_empty() || k == 0 {
      return Ok(Vec::new());
    }
    let embeddings = self.engine.embed(&[formatted.as_str()]).await?;
    let Some(qvec) = embeddings.into_iter().next() else {
      return Ok(Vec::new());
    };
    let raw = self.index.search(&qvec, k)?;
    let mut hits = Vec::with_capacity(raw.len());
    for (id, neural) in raw {
      if let Some(document) = self.index.get(&id).cloned() {
        hits.push(SearchHit {
          document,
          neural_score: neural,
          semantic_score: None,
          score: neural,
        });
      }
    }
    Ok(hits)
  }

  /// Neural search fused with [`SemanticBoost`]: `0.7 * neural + 0.3 * semantic`.
  pub async fn search_fused(
    &self,
    query: &str,
    k: usize,
    natural_language: bool,
    boost: &dyn SemanticBoost,
  ) -> EmbeddingResult<Vec<SearchHit>> {
    // Over-fetch so re-ranking has room.
    let fetch = k.saturating_mul(3).max(k);
    let mut hits = self.search(query, fetch, natural_language).await?;
    for hit in &mut hits {
      let sem = boost.score(&hit.document.id).unwrap_or(0.0);
      hit.semantic_score = Some(sem);
      hit.score = fuse_scores(hit.neural_score, sem);
    }
    hits.sort_by(|a, b| {
      b.score
        .partial_cmp(&a.score)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(k);
    Ok(hits)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::mock::MockEmbeddingEngine;
  use rustc_hash::FxHashMap;

  #[tokio::test]
  async fn search_finds_indexed_doc() {
    let engine: Arc<dyn EmbeddingEngine> = Arc::new(MockEmbeddingEngine::default());
    let mut searcher = CodeSearcher::new(Arc::clone(&engine)).unwrap();
    let doc = CodeDocument {
      id: "a::foo#0".into(),
      qualified_name: "a::foo".into(),
      name: "foo".into(),
      label: "Function".into(),
      file_path: Some("a.rs".into()),
      start_line: 1,
      end_line: 10,
      chunk_index: 0,
      text: "fn foo() { authenticate_user(); }".into(),
      metadata: Default::default(),
    };
    searcher.index_documents(&[doc]).await.unwrap();
    let hits = searcher
      .search("authenticate_user", 5, false)
      .await
      .unwrap();
    assert!(!hits.is_empty());
    assert_eq!(hits[0].document.qualified_name, "a::foo");
  }

  #[tokio::test]
  async fn fused_reranks() {
    let engine: Arc<dyn EmbeddingEngine> = Arc::new(MockEmbeddingEngine::default());
    let mut searcher = CodeSearcher::new(engine).unwrap();
    let docs = vec![
      CodeDocument {
        id: "low#0".into(),
        qualified_name: "low".into(),
        name: "low".into(),
        label: "Function".into(),
        file_path: None,
        start_line: 1,
        end_line: 1,
        chunk_index: 0,
        text: "completely unrelated zebra pineapple".into(),
        metadata: Default::default(),
      },
      CodeDocument {
        id: "high#0".into(),
        qualified_name: "high".into(),
        name: "high".into(),
        label: "Function".into(),
        file_path: None,
        start_line: 1,
        end_line: 1,
        chunk_index: 0,
        text: "auth middleware login session".into(),
        metadata: Default::default(),
      },
    ];
    searcher.index_documents(&docs).await.unwrap();
    let mut map = FxHashMap::default();
    map.insert("high".into(), 1.0_f32);
    map.insert("low".into(), 0.0_f32);
    let boost = MapBoost(map);
    let hits = searcher
      .search_fused("auth middleware", 2, true, &boost)
      .await
      .unwrap();
    assert!(!hits.is_empty());
    assert!(hits[0].semantic_score.is_some());
  }
}
