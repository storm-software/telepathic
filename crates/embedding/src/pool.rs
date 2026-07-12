//! Mean pooling and L2 normalization for token embeddings.

const DENOM_EPS: f32 = 1e-10;

/// Mean-pool token embeddings with an attention mask.
pub fn mean_pool(
  output_data: &[f32],
  seq_len: usize,
  hidden_dim: usize,
  attention_mask: &[i64],
  output_dim: usize,
) -> Vec<f32> {
  let mut pooled = vec![0.0_f32; output_dim];
  let dim = output_dim.min(hidden_dim);

  for s in 0..seq_len {
    if s < attention_mask.len() && attention_mask[s] == 1 {
      for (h, pooled_val) in pooled.iter_mut().enumerate().take(dim) {
        let idx = s * hidden_dim + h;
        if idx < output_data.len() {
          *pooled_val += output_data[idx];
        }
      }
    }
  }

  #[expect(clippy::cast_precision_loss)]
  let real_tokens = attention_mask.iter().filter(|&&m| m == 1).count().max(1) as f32;
  for val in &mut pooled {
    *val /= real_tokens;
  }
  pooled
}

/// L2-normalize to unit length.
pub fn l2_normalize(vec: &[f32]) -> Vec<f32> {
  let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
  if norm > DENOM_EPS {
    vec.iter().map(|x| x / norm).collect()
  } else {
    vec.to_vec()
  }
}

/// Cosine similarity between two dense vectors (same length).
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
  let len = a.len().min(b.len());
  if len == 0 {
    return 0.0;
  }
  let mut dot = 0.0_f32;
  let mut mag_a = 0.0_f32;
  let mut mag_b = 0.0_f32;
  for i in 0..len {
    dot += a[i] * b[i];
    mag_a += a[i] * a[i];
    mag_b += b[i] * b[i];
  }
  let denom = mag_a.sqrt() * mag_b.sqrt();
  if denom < DENOM_EPS {
    0.0
  } else {
    dot / denom
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn l2_unit_vector() {
    let n = l2_normalize(&[3.0, 4.0]);
    let norm: f32 = n.iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!((norm - 1.0).abs() < 1e-5);
  }

  #[test]
  fn cosine_identical() {
    let v = l2_normalize(&[1.0, 2.0, 3.0]);
    assert!((cosine(&v, &v) - 1.0).abs() < 1e-5);
  }

  #[test]
  fn mean_pool_masks_padding() {
    // seq=2, hidden=2: token0=[1,1], token1=[10,10]; mask only token0
    let data = [1.0, 1.0, 10.0, 10.0];
    let mask = [1_i64, 0];
    let pooled = mean_pool(&data, 2, 2, &mask, 2);
    assert!((pooled[0] - 1.0).abs() < 1e-5);
    assert!((pooled[1] - 1.0).abs() < 1e-5);
  }
}
