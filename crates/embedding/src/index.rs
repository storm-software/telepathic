//! Pure-Rust HNSW index (cosine). Avoids `usearch`/`cxx` clash with `lbug`.

use rustc_hash::FxHashMap;

use crate::document::CodeDocument;
use crate::error::{EmbeddingError, EmbeddingResult};
use crate::pool::cosine;

const DEFAULT_M: usize = 16;
const DEFAULT_EF_CONSTRUCTION: usize = 64;
const DEFAULT_EF_SEARCH: usize = 64;

#[derive(Clone)]
struct Node {
  id: String,
  vector: Vec<f32>,
  /// Neighbors per layer (layer 0 = base).
  neighbors: Vec<Vec<usize>>,
}

/// Dense vector HNSW index keyed by document id.
pub struct HnswIndex {
  dimensions: usize,
  m: usize,
  ef_construction: usize,
  ef_search: usize,
  nodes: Vec<Node>,
  docs: FxHashMap<String, CodeDocument>,
  id_to_idx: FxHashMap<String, usize>,
  entry_point: Option<usize>,
  max_layer: usize,
}

impl HnswIndex {
  pub fn new(dimensions: usize) -> EmbeddingResult<Self> {
    Ok(Self {
      dimensions: dimensions.max(1),
      m: DEFAULT_M,
      ef_construction: DEFAULT_EF_CONSTRUCTION,
      ef_search: DEFAULT_EF_SEARCH,
      nodes: Vec::new(),
      docs: FxHashMap::default(),
      id_to_idx: FxHashMap::default(),
      entry_point: None,
      max_layer: 0,
    })
  }

  pub fn dimensions(&self) -> usize {
    self.dimensions
  }

  pub fn len(&self) -> usize {
    self.docs.len()
  }

  pub fn is_empty(&self) -> bool {
    self.docs.is_empty()
  }

  pub fn upsert(&mut self, doc: CodeDocument, vector: &[f32]) -> EmbeddingResult<()> {
    if vector.len() != self.dimensions {
      return Err(EmbeddingError::Index(format!(
        "vector dim {} != index dim {}",
        vector.len(),
        self.dimensions
      )));
    }

    if let Some(&old_idx) = self.id_to_idx.get(&doc.id) {
      self.nodes[old_idx].vector = vector.to_vec();
      self.docs.insert(doc.id.clone(), doc);
      return Ok(());
    }

    let layer = self.random_level();
    let idx = self.nodes.len();
    self.nodes.push(Node {
      id: doc.id.clone(),
      vector: vector.to_vec(),
      neighbors: vec![Vec::new(); layer + 1],
    });
    self.docs.insert(doc.id.clone(), doc.clone());
    self.id_to_idx.insert(doc.id, idx);

    if self.entry_point.is_none() {
      self.entry_point = Some(idx);
      self.max_layer = layer;
      return Ok(());
    }

    let mut ep = self.entry_point.expect("entry set");
    if self.max_layer > layer {
      for l in (layer + 1..=self.max_layer).rev() {
        ep = self.greedy_search(vector, ep, l);
      }
    }

    for l in (0..=layer.min(self.max_layer)).rev() {
      let candidates = self.search_layer(vector, ep, self.ef_construction, l);
      let selected = select_neighbors(&candidates, self.m);
      for &nb in &selected {
        link(&mut self.nodes, idx, nb, l, self.m);
      }
      if let Some(&(best, _)) = candidates.first() {
        ep = best;
      }
    }

    if layer > self.max_layer {
      self.max_layer = layer;
      self.entry_point = Some(idx);
    }

    Ok(())
  }

  pub fn search(&self, query: &[f32], k: usize) -> EmbeddingResult<Vec<(String, f32)>> {
    if query.len() != self.dimensions {
      return Err(EmbeddingError::Index(format!(
        "query dim {} != index dim {}",
        query.len(),
        self.dimensions
      )));
    }
    if k == 0 || self.is_empty() {
      return Ok(Vec::new());
    }

    let mut ep = self.entry_point.expect("non-empty");
    for l in (1..=self.max_layer).rev() {
      ep = self.greedy_search(query, ep, l);
    }
    let mut candidates = self.search_layer(query, ep, self.ef_search.max(k), 0);
    candidates.sort_by(|a, b| {
      b.1.partial_cmp(&a.1)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    candidates.truncate(k);

    Ok(
      candidates
        .into_iter()
        .map(|(idx, score)| (self.nodes[idx].id.clone(), score))
        .collect(),
    )
  }

  pub fn get(&self, id: &str) -> Option<&CodeDocument> {
    self.docs.get(id)
  }

  fn random_level(&self) -> usize {
    let mut lvl = 0_usize;
    let mut x = (self.nodes.len() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    while lvl < 16 {
      x ^= x << 13;
      x ^= x >> 7;
      x ^= x << 17;
      if !x.is_multiple_of(self.m as u64) {
        break;
      }
      lvl += 1;
    }
    lvl
  }

  fn greedy_search(&self, query: &[f32], mut ep: usize, layer: usize) -> usize {
    loop {
      let mut changed = false;
      if layer >= self.nodes[ep].neighbors.len() {
        break;
      }
      let mut best = cosine(query, &self.nodes[ep].vector);
      for &nb in &self.nodes[ep].neighbors[layer] {
        let score = cosine(query, &self.nodes[nb].vector);
        if score > best {
          best = score;
          ep = nb;
          changed = true;
        }
      }
      if !changed {
        break;
      }
    }
    ep
  }

  fn search_layer(
    &self,
    query: &[f32],
    entry: usize,
    ef: usize,
    layer: usize,
  ) -> Vec<(usize, f32)> {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    #[derive(Clone, PartialEq)]
    struct Cand {
      idx: usize,
      score: f32,
    }
    impl Eq for Cand {}
    impl PartialOrd for Cand {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }
    impl Ord for Cand {
      fn cmp(&self, other: &Self) -> Ordering {
        self
          .score
          .partial_cmp(&other.score)
          .unwrap_or(Ordering::Equal)
      }
    }

    let mut visited = vec![false; self.nodes.len()];
    // Max-heap of candidates to explore.
    let mut candidates = BinaryHeap::new();
    // Min-heap of results via inverted score.
    let mut w: BinaryHeap<Cand> = BinaryHeap::new();

    let entry_score = cosine(query, &self.nodes[entry].vector);
    visited[entry] = true;
    candidates.push(Cand { idx: entry, score: entry_score });
    // Store as negative for min-heap behavior on score.
    w.push(Cand {
      idx: entry,
      score: -entry_score,
    });

    while let Some(curr) = candidates.pop() {
      let worst = w.peek().map(|c| -c.score).unwrap_or(f32::NEG_INFINITY);
      if curr.score < worst && w.len() >= ef {
        break;
      }
      if layer >= self.nodes[curr.idx].neighbors.len() {
        continue;
      }
      for &nb in &self.nodes[curr.idx].neighbors[layer] {
        if visited[nb] {
          continue;
        }
        visited[nb] = true;
        let score = cosine(query, &self.nodes[nb].vector);
        let worst = w.peek().map(|c| -c.score).unwrap_or(f32::NEG_INFINITY);
        if score > worst || w.len() < ef {
          candidates.push(Cand { idx: nb, score });
          w.push(Cand {
            idx: nb,
            score: -score,
          });
          if w.len() > ef {
            let _ = w.pop();
          }
        }
      }
    }

    let mut out: Vec<(usize, f32)> = w.into_iter().map(|c| (c.idx, -c.score)).collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    out
  }
}

fn select_neighbors(candidates: &[(usize, f32)], m: usize) -> Vec<usize> {
  let mut sorted = candidates.to_vec();
  sorted.sort_by(|a, b| {
    b.1.partial_cmp(&a.1)
      .unwrap_or(std::cmp::Ordering::Equal)
  });
  sorted.into_iter().take(m).map(|(i, _)| i).collect()
}

fn link(nodes: &mut [Node], a: usize, b: usize, layer: usize, m: usize) {
  if layer < nodes[a].neighbors.len() && !nodes[a].neighbors[layer].contains(&b) {
    nodes[a].neighbors[layer].push(b);
    if nodes[a].neighbors[layer].len() > m {
      shrink_neighbors(nodes, a, layer, m);
    }
  }
  if layer < nodes[b].neighbors.len() && !nodes[b].neighbors[layer].contains(&a) {
    nodes[b].neighbors[layer].push(a);
    if nodes[b].neighbors[layer].len() > m {
      shrink_neighbors(nodes, b, layer, m);
    }
  }
}

fn shrink_neighbors(nodes: &mut [Node], idx: usize, layer: usize, m: usize) {
  let vec = nodes[idx].vector.clone();
  let scores: Vec<(usize, f32)> = nodes[idx].neighbors[layer]
    .iter()
    .map(|&j| (j, cosine(&vec, &nodes[j].vector)))
    .collect();
  nodes[idx].neighbors[layer] = select_neighbors(&scores, m);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hnsw_returns_nearest() {
    let mut index = HnswIndex::new(4).unwrap();
    let docs = [
      ("a", vec![1.0, 0.0, 0.0, 0.0]),
      ("b", vec![0.0, 1.0, 0.0, 0.0]),
      ("c", vec![0.9, 0.1, 0.0, 0.0]),
    ];
    for (id, v) in docs {
      let doc = CodeDocument {
        id: id.into(),
        qualified_name: id.into(),
        name: id.into(),
        label: "Function".into(),
        file_path: None,
        start_line: 1,
        end_line: 1,
        chunk_index: 0,
        text: id.into(),
        metadata: Default::default(),
      };
      index.upsert(doc, &v).unwrap();
    }
    let hits = index.search(&[1.0, 0.0, 0.0, 0.0], 2).unwrap();
    assert_eq!(hits[0].0, "a");
  }
}
