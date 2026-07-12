//! Provider-agnostic embedding configuration.

#[cfg(feature = "onnx")]
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::LOCAL_DIM;
use crate::engine::EmbeddingEngine;
use crate::error::{EmbeddingError, EmbeddingResult};
use crate::mock::MockEmbeddingEngine;

#[cfg(feature = "onnx")]
use crate::onnx::OnnxEmbeddingEngine;
#[cfg(feature = "voyage")]
use crate::voyage::VoyageEmbeddingEngine;

/// Selected embedding backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProvider {
  #[default]
  Onnx,
  Voyage,
  Mock,
}

/// Top-level embedding config (`from_env` / programmatic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
  pub provider: EmbeddingProvider,
  pub model: String,
  pub dimensions: usize,
  pub batch_size: usize,
  pub endpoint: Option<String>,
  pub api_key: Option<String>,
  pub mock: bool,
  #[cfg(feature = "onnx")]
  pub onnx: OnnxEmbeddingConfig,
}

impl Default for EmbeddingConfig {
  fn default() -> Self {
    Self {
      provider: EmbeddingProvider::Onnx,
      model: "coderankembed".into(),
      dimensions: LOCAL_DIM,
      batch_size: 32,
      endpoint: None,
      api_key: None,
      mock: false,
      #[cfg(feature = "onnx")]
      onnx: OnnxEmbeddingConfig::coderankembed("./target/models"),
    }
  }
}

impl EmbeddingConfig {
  /// Read `EMBEDDING_PROVIDER`, `MOCK_EMBEDDING`, `EMBEDDING_MODEL`, etc.
  pub fn from_env() -> Self {
    let mut cfg = Self::default();
    if env_truthy("MOCK_EMBEDDING") {
      cfg.mock = true;
      cfg.provider = EmbeddingProvider::Mock;
    }
    if let Ok(p) = std::env::var("EMBEDDING_PROVIDER") {
      cfg.provider = match p.to_ascii_lowercase().as_str() {
        "voyage" | "voyageai" => EmbeddingProvider::Voyage,
        "mock" => EmbeddingProvider::Mock,
        "onnx" | "coderank" | "coderankembed" => EmbeddingProvider::Onnx,
        other => {
          tracing::warn!(provider = %other, "unknown EMBEDDING_PROVIDER; using onnx");
          EmbeddingProvider::Onnx
        }
      };
    }
    if let Ok(model) = std::env::var("EMBEDDING_MODEL") {
      cfg.model = model;
    }
    if let Ok(dim) = std::env::var("EMBEDDING_DIMENSIONS") {
      if let Ok(d) = dim.parse::<usize>() {
        cfg.dimensions = d;
      }
    }
    if let Ok(bs) = std::env::var("EMBEDDING_BATCH_SIZE") {
      if let Ok(b) = bs.parse::<usize>() {
        cfg.batch_size = b;
      }
    }
    if let Ok(ep) = std::env::var("EMBEDDING_ENDPOINT") {
      cfg.endpoint = Some(ep);
    }
    if let Ok(key) = std::env::var("EMBEDDING_API_KEY")
      .or_else(|_| std::env::var("VOYAGE_API_KEY"))
      .or_else(|_| std::env::var("LLM_API_KEY"))
    {
      cfg.api_key = Some(key);
    }
    cfg
  }

  /// Build an [`EmbeddingEngine`] for this config.
  #[cfg_attr(
    not(any(feature = "onnx", feature = "voyage")),
    expect(clippy::unused_async)
  )]
  #[cfg_attr(
    not(any(feature = "onnx", feature = "voyage")),
    expect(clippy::unused_async_trait_impl)
  )]
  pub async fn create_engine(&self) -> EmbeddingResult<Arc<dyn EmbeddingEngine>> {
    if self.mock || self.provider == EmbeddingProvider::Mock {
      return Ok(Arc::new(MockEmbeddingEngine::new(self.dimensions)));
    }
    match self.provider {
      EmbeddingProvider::Mock => Ok(Arc::new(MockEmbeddingEngine::new(self.dimensions))),
      EmbeddingProvider::Onnx => {
        #[cfg(feature = "onnx")]
        {
          let engine = OnnxEmbeddingEngine::with_auto_download(self.onnx.clone()).await?;
          Ok(Arc::new(engine))
        }
        #[cfg(not(feature = "onnx"))]
        {
          Err(EmbeddingError::NotEnabled(
            "rebuild with `--features onnx`".into(),
          ))
        }
      }
      EmbeddingProvider::Voyage => {
        #[cfg(feature = "voyage")]
        {
          let engine = VoyageEmbeddingEngine::new(
            self.api_key.clone(),
            self.endpoint.clone(),
            self.model.clone(),
            self.dimensions,
            self.batch_size,
          )?;
          Ok(Arc::new(engine))
        }
        #[cfg(not(feature = "voyage"))]
        {
          Err(EmbeddingError::NotEnabled(
            "rebuild with `--features voyage`".into(),
          ))
        }
      }
    }
  }
}

fn env_truthy(name: &str) -> bool {
  std::env::var(name).is_ok_and(|v| {
    matches!(
      v.to_ascii_lowercase().as_str(),
      "1" | "true" | "yes" | "on"
    )
  })
}

/// ONNX-specific settings (CodeRankEmbed by default).
#[cfg(feature = "onnx")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnnxEmbeddingConfig {
  pub model_path: PathBuf,
  pub tokenizer_path: PathBuf,
  pub model_name: String,
  pub dimensions: usize,
  pub max_sequence_length: usize,
  pub batch_size: usize,
}

#[cfg(feature = "onnx")]
impl Default for OnnxEmbeddingConfig {
  fn default() -> Self {
    Self::coderankembed("./target/models")
  }
}

#[cfg(feature = "onnx")]
impl OnnxEmbeddingConfig {
  /// Default local model: nomic-ai/CodeRankEmbed (137M, 768-d) INT8 ONNX.
  pub fn coderankembed(model_dir: impl Into<PathBuf>) -> Self {
    let base = model_dir.into();
    Self {
      model_path: base.join("CodeRankEmbed-model_quantized.onnx"),
      tokenizer_path: base.join("coderankembed-tokenizer.json"),
      model_name: "coderankembed".into(),
      dimensions: LOCAL_DIM,
      // Chunker uses 256; model supports up to 8192 — keep inference window modest.
      max_sequence_length: 512,
      batch_size: 16,
    }
  }
}
