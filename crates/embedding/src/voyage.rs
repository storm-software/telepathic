//! Voyage AI HTTP embedding engine (`voyage-code-3`).

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::VOYAGE_CODE3_DIM;
use crate::engine::EmbeddingEngine;
use crate::error::{EmbeddingError, EmbeddingResult};
use crate::pool::l2_normalize;

const DEFAULT_ENDPOINT: &str = "https://api.voyageai.com/v1/embeddings";
const DEFAULT_MODEL: &str = "voyage-code-3";

/// Voyage Code 3 (1024-d) embedding client.
pub struct VoyageEmbeddingEngine {
  client: reqwest::Client,
  api_key: String,
  endpoint: String,
  model: String,
  dimensions: usize,
  batch_size: usize,
}

impl VoyageEmbeddingEngine {
  pub fn new(
    api_key: Option<String>,
    endpoint: Option<String>,
    model: String,
    dimensions: usize,
    batch_size: usize,
  ) -> EmbeddingResult<Self> {
    let api_key = api_key
      .or_else(|| std::env::var("VOYAGE_API_KEY").ok())
      .or_else(|| std::env::var("EMBEDDING_API_KEY").ok())
      .ok_or_else(|| {
        EmbeddingError::Config("VOYAGE_API_KEY / EMBEDDING_API_KEY required".into())
      })?;

    let model = if model.is_empty() || model == "coderankembed" {
      DEFAULT_MODEL.to_owned()
    } else {
      model
    };

    let dimensions = if dimensions == 0 || dimensions == crate::LOCAL_DIM {
      VOYAGE_CODE3_DIM
    } else {
      dimensions
    };

    Ok(Self {
      client: reqwest::Client::new(),
      api_key,
      endpoint: endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned()),
      model,
      dimensions,
      batch_size: batch_size.max(1),
    })
  }

  async fn embed_batch(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>> {
    #[derive(Deserialize)]
    struct Item {
      embedding: Vec<f32>,
      index: usize,
    }
    #[derive(Deserialize)]
    struct Response {
      data: Vec<Item>,
    }

    let body = json!({
      "input": texts,
      "model": self.model,
      "input_type": "document",
    });

    let response = self
      .client
      .post(&self.endpoint)
      .bearer_auth(&self.api_key)
      .json(&body)
      .send()
      .await
      .map_err(|e| EmbeddingError::Http(e.to_string()))?;

    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.unwrap_or_default();
      return Err(EmbeddingError::Api(format!("{status}: {text}")));
    }

    let parsed: Response = response
      .json()
      .await
      .map_err(|e| EmbeddingError::Api(e.to_string()))?;

    let mut ordered = vec![Vec::new(); texts.len()];
    for item in parsed.data {
      if item.index < ordered.len() {
        ordered[item.index] = l2_normalize(&item.embedding);
      }
    }
    if ordered.iter().any(Vec::is_empty) {
      return Err(EmbeddingError::Api(
        "voyage response missing embedding slots".into(),
      ));
    }
    Ok(ordered)
  }
}

#[async_trait]
impl EmbeddingEngine for VoyageEmbeddingEngine {
  async fn embed(&self, texts: &[&str]) -> EmbeddingResult<Vec<Vec<f32>>> {
    if texts.is_empty() {
      return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(texts.len());
    for chunk in texts.chunks(self.batch_size) {
      out.extend(self.embed_batch(chunk).await?);
    }
    Ok(out)
  }

  fn dimension(&self) -> usize {
    self.dimensions
  }

  fn batch_size(&self) -> usize {
    self.batch_size
  }

  fn max_sequence_length(&self) -> usize {
    32_000
  }
}
