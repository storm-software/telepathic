//! ONNX Runtime engine for CodeRankEmbed (mean-pool + L2 normalize).

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Tensor;
use tokenizers::Tokenizer;

use crate::config::OnnxEmbeddingConfig;
use crate::download::{ModelUrls, ensure_model_exists, ensure_tokenizer_exists};
use crate::engine::EmbeddingEngine;
use crate::error::{EmbeddingError, EmbeddingResult};
use crate::pool::{l2_normalize, mean_pool};

type TokenizationBatch = (Vec<Vec<i64>>, Vec<Vec<i64>>);

/// Local ONNX embedding engine.
pub struct OnnxEmbeddingEngine {
  session: Arc<Mutex<Session>>,
  tokenizer: Arc<Mutex<Tokenizer>>,
  config: OnnxEmbeddingConfig,
}

impl OnnxEmbeddingEngine {
  /// Load model + tokenizer from paths already on disk.
  pub fn new(config: OnnxEmbeddingConfig) -> EmbeddingResult<Self> {
    let _ = ort::init().commit();

    if !config.model_path.exists() {
      return Err(EmbeddingError::ModelLoad(format!(
        "model file not found: {}",
        config.model_path.display()
      )));
    }

    let tokenizer = Tokenizer::from_file(&config.tokenizer_path).map_err(|e| {
      EmbeddingError::Tokenizer(format!(
        "load tokenizer {}: {e}",
        config.tokenizer_path.display()
      ))
    })?;

    let session = Session::builder()
      .map_err(|e| EmbeddingError::ModelLoad(e.to_string()))?
      .with_optimization_level(GraphOptimizationLevel::Level3)
      .map_err(|e| EmbeddingError::ModelLoad(e.to_string()))?
      .commit_from_file(&config.model_path)
      .map_err(|e| EmbeddingError::ModelLoad(e.to_string()))?;

    tracing::info!(
      model = %config.model_name,
      dims = config.dimensions,
      "loaded ONNX embedding engine"
    );

    Ok(Self {
      session: Arc::new(Mutex::new(session)),
      tokenizer: Arc::new(Mutex::new(tokenizer)),
      config,
    })
  }

  /// Download CodeRankEmbed artifacts when missing, then [`Self::new`].
  pub async fn with_auto_download(config: OnnxEmbeddingConfig) -> EmbeddingResult<Self> {
    let urls = match config.model_name.to_ascii_lowercase().as_str() {
      "coderankembed" | "coderank" | "nomic-coderankembed" | "code-rank-embed" => {
        ModelUrls::CODERANKEMBED
      }
      other => {
        return Err(EmbeddingError::ModelLoad(format!(
          "unknown ONNX model '{other}'. supported: coderankembed"
        )));
      }
    };

    ensure_model_exists(&config.model_path, urls.model_url).await?;
    ensure_tokenizer_exists(&config.tokenizer_path, urls.tokenizer_url).await?;
    Self::new(config)
  }

  fn tokenize_batch(&self, texts: &[&str]) -> EmbeddingResult<TokenizationBatch> {
    let tokenizer = self
      .tokenizer
      .lock()
      .map_err(|_| EmbeddingError::Tokenizer("tokenizer lock poisoned".into()))?;
    let max_len = self.config.max_sequence_length;

    let mut input_ids_batch = Vec::with_capacity(texts.len());
    let mut attention_mask_batch = Vec::with_capacity(texts.len());

    for text in texts {
      let encoding = tokenizer
        .encode(*text, true)
        .map_err(|e| EmbeddingError::Tokenizer(e.to_string()))?;

      let mut ids: Vec<i64> = encoding.get_ids().iter().map(|&id| i64::from(id)).collect();
      let mut mask: Vec<i64> = encoding
        .get_attention_mask()
        .iter()
        .map(|&m| i64::from(m))
        .collect();

      if ids.len() > max_len {
        ids.truncate(max_len);
        mask.truncate(max_len);
      }
      while ids.len() < max_len {
        ids.push(0);
        mask.push(0);
      }

      input_ids_batch.push(ids);
      attention_mask_batch.push(mask);
    }

    Ok((input_ids_batch, attention_mask_batch))
  }

  fn extract_embedding(
    &self,
    output_data: &[f32],
    output_shape: &[usize],
    attention_mask: &[i64],
  ) -> EmbeddingResult<Vec<f32>> {
    let output_dim = self.config.dimensions;

    if output_shape.len() == 3 {
      let seq_len = output_shape[1];
      let hidden_dim = output_shape[2];
      let pooled = mean_pool(output_data, seq_len, hidden_dim, attention_mask, output_dim);
      Ok(l2_normalize(&pooled))
    } else if output_shape.len() == 2 {
      let embedding: Vec<f32> = output_data.iter().take(output_dim).copied().collect();
      Ok(l2_normalize(&embedding))
    } else {
      Err(EmbeddingError::Inference(format!(
        "unexpected output shape: {output_shape:?}"
      )))
    }
  }

  async fn embed_batch(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>> {
    if texts.is_empty() {
      return Ok(Vec::new());
    }

    let batch_size = texts.len();
    let seq_len = self.config.max_sequence_length;
    let (input_ids_batch, attention_mask_batch) = self.tokenize_batch(texts)?;

    let input_ids_flat: Vec<i64> = input_ids_batch.iter().flatten().copied().collect();
    let attention_mask_flat: Vec<i64> = attention_mask_batch.iter().flatten().copied().collect();
    let token_type_flat = vec![0_i64; batch_size * seq_len];
    let attention_masks = attention_mask_batch;

    let session = Arc::clone(&self.session);

    let (output_shape, output_data) = tokio::task::spawn_blocking(move || {
      let mut session = session
        .lock()
        .map_err(|_| EmbeddingError::Inference("session lock poisoned".into()))?;

      let input_ids_tensor = Tensor::from_array(([batch_size, seq_len], input_ids_flat))
        .map_err(|e| EmbeddingError::Inference(e.to_string()))?;
      let attention_mask_tensor =
        Tensor::from_array(([batch_size, seq_len], attention_mask_flat))
          .map_err(|e| EmbeddingError::Inference(e.to_string()))?;
      let token_type_ids_tensor = Tensor::from_array(([batch_size, seq_len], token_type_flat))
        .map_err(|e| EmbeddingError::Inference(e.to_string()))?;

      let outputs = session
        .run(ort::inputs! {
          "input_ids" => input_ids_tensor,
          "attention_mask" => attention_mask_tensor,
          "token_type_ids" => token_type_ids_tensor,
        })
        .map_err(|e| {
          EmbeddingError::Inference(format!(
            "ONNX run failed (need input_ids, attention_mask, token_type_ids): {e}"
          ))
        })?;

      let (_name, tensor) = outputs
        .iter()
        .next()
        .ok_or_else(|| EmbeddingError::Inference("no ONNX outputs".into()))?;

      let (shape, data) = tensor
        .try_extract_tensor::<f32>()
        .map_err(|e| EmbeddingError::Inference(e.to_string()))?;
      let shape_usize: Vec<usize> = shape
        .iter()
        .map(|&d| usize::try_from(d).unwrap_or(0))
        .collect();
      Ok::<_, EmbeddingError>((shape_usize, data.to_vec()))
    })
    .await
    .map_err(|e| EmbeddingError::Inference(e.to_string()))??;

    let mut embeddings = Vec::with_capacity(batch_size);

    if output_shape.len() == 3 {
      let seq = output_shape[1];
      let hidden = output_shape[2];
      let sample_size = seq * hidden;
      for (i, mask) in attention_masks.iter().enumerate().take(batch_size) {
        let start = i * sample_size;
        let end = start + sample_size;
        let sample = &output_data[start..end];
        embeddings.push(self.extract_embedding(sample, &[1, seq, hidden], mask)?);
      }
    } else if output_shape.len() == 2 {
      let hidden = output_shape[1];
      for (i, mask) in attention_masks.iter().enumerate().take(batch_size) {
        let start = i * hidden;
        let end = start + hidden;
        let sample = &output_data[start..end.min(output_data.len())];
        embeddings.push(self.extract_embedding(sample, &[1, hidden], mask)?);
      }
    } else {
      return Err(EmbeddingError::Inference(format!(
        "unexpected output shape {output_shape:?}"
      )));
    }

    Ok(embeddings)
  }
}

#[async_trait]
impl EmbeddingEngine for OnnxEmbeddingEngine {
  async fn embed(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>> {
    if texts.is_empty() {
      return Ok(Vec::new());
    }
    let batch = self.config.batch_size.max(1);
    if texts.len() <= batch {
      return self.embed_batch(texts).await;
    }
    let mut out = Vec::with_capacity(texts.len());
    for chunk in texts.chunks(batch) {
      out.extend(self.embed_batch(chunk).await?);
    }
    Ok(out)
  }

  fn dimension(&self) -> usize {
    self.config.dimensions
  }

  fn batch_size(&self) -> usize {
    self.config.batch_size
  }

  fn max_sequence_length(&self) -> usize {
    self.config.max_sequence_length
  }
}
