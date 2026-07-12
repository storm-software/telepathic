use async_trait::async_trait;
use xxhash_rust::xxh3::xxh3_64;

use crate::LOCAL_DIM;
use crate::engine::EmbeddingEngine;
use crate::error::EmbeddingResult;
use crate::pool::l2_normalize;

/// Deterministic mock engine for tests (no model download).
#[derive(Debug, Clone)]
pub struct MockEmbeddingEngine {
  dim: usize,
  batch: usize,
}

impl Default for MockEmbeddingEngine {
  fn default() -> Self {
    Self { dim: LOCAL_DIM, batch: 32 }
  }
}

impl MockEmbeddingEngine {
  pub fn new(dim: usize) -> Self {
    Self { dim: dim.max(1), batch: 32 }
  }

  fn embed_one(&self, text: &str) -> Vec<f32> {
    let mut v = vec![0.0_f32; self.dim];
    if text.is_empty() {
      return v;
    }
    let seed = xxh3_64(text.as_bytes());
    for (i, slot) in v.iter_mut().enumerate() {
      let h = xxh3_64(&(seed.wrapping_add(i as u64)).to_le_bytes());
      let bit = if h & 1 == 1 { 1.0_f32 } else { -1.0_f32 };
      #[expect(clippy::cast_precision_loss)]
      let scale = ((h >> 1) % 1000) as f32 / 1000.0;
      *slot = bit * scale;
    }
    l2_normalize(&v)
  }
}

#[async_trait]
impl EmbeddingEngine for MockEmbeddingEngine {
  async fn embed(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>> {
    Ok(texts.iter().map(|t| self.embed_one(t)).collect())
  }

  fn dimension(&self) -> usize {
    self.dim
  }

  fn batch_size(&self) -> usize {
    self.batch
  }

  fn max_sequence_length(&self) -> usize {
    8192
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn mock_is_deterministic() {
    let engine = MockEmbeddingEngine::default();
    let a = engine.embed(&["hello"]).await.unwrap();
    let b = engine.embed(&["hello"]).await.unwrap();
    assert_eq!(a, b);
    assert_eq!(a[0].len(), LOCAL_DIM);
  }
}
