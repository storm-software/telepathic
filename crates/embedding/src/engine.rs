use async_trait::async_trait;

use crate::error::EmbeddingResult;

/// Async text → dense embedding engine.
///
/// All returned vectors are L2-normalized for cosine similarity.
#[async_trait]
pub trait EmbeddingEngine: Send + Sync {
  /// Embed a batch of texts into unit vectors.
  async fn embed(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>>;

  /// Output dimensionality.
  fn dimension(&self) -> usize;

  /// Preferred batch size for [`embed`](Self::embed).
  fn batch_size(&self) -> usize;

  /// Max tokens per input (truncate beyond this).
  fn max_sequence_length(&self) -> usize;
}
