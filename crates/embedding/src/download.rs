//! Lazy download of CodeRankEmbed ONNX + tokenizer from HuggingFace Hub.

use std::path::{Path, PathBuf};

use crate::error::{EmbeddingError, EmbeddingResult};

/// HuggingFace Hub URLs for supported ONNX models.
pub struct ModelUrls {
  pub model_url: &'static str,
  pub tokenizer_url: &'static str,
}

impl ModelUrls {
  /// INT8 CodeRankEmbed (768-d) — default local engine.
  ///
  /// Model: `mrsladoje/CodeRankEmbed-onnx-int8`
  /// Tokenizer: upstream `nomic-ai/CodeRankEmbed`
  pub const CODERANKEMBED: ModelUrls = ModelUrls {
    model_url: "https://huggingface.co/mrsladoje/CodeRankEmbed-onnx-int8/resolve/main/onnx/model.onnx",
    tokenizer_url: "https://huggingface.co/nomic-ai/CodeRankEmbed/resolve/main/tokenizer.json",
  };
}

async fn download_file(url: &str, dest: &Path) -> EmbeddingResult<()> {
  if let Some(parent) = dest.parent() {
    tokio::fs::create_dir_all(parent).await?;
  }

  let response = reqwest::get(url)
    .await
    .map_err(|e| EmbeddingError::ModelLoad(format!("download {url}: {e}")))?;

  if !response.status().is_success() {
    return Err(EmbeddingError::ModelLoad(format!(
      "download {url}: HTTP {}",
      response.status()
    )));
  }

  let bytes = response
    .bytes()
    .await
    .map_err(|e| EmbeddingError::ModelLoad(format!("read body: {e}")))?;

  tokio::fs::write(dest, &bytes).await?;
  Ok(())
}

/// Ensure model file exists; download when missing. Returns `true` if downloaded.
pub async fn ensure_model_exists(path: &Path, url: &str) -> EmbeddingResult<bool> {
  if path.exists() {
    return Ok(false);
  }
  tracing::info!(?path, url, "downloading embedding model");
  download_file(url, path).await?;
  Ok(true)
}

/// Ensure tokenizer.json exists; download when missing.
pub async fn ensure_tokenizer_exists(path: &Path, url: &str) -> EmbeddingResult<bool> {
  if path.exists() {
    return Ok(false);
  }
  tracing::info!(?path, url, "downloading tokenizer");
  download_file(url, path).await?;
  Ok(true)
}

/// Download model + tokenizer for a known name into `model_dir`.
pub async fn download_model(
  model_name: &str,
  model_dir: &Path,
) -> EmbeddingResult<(PathBuf, PathBuf)> {
  let key = model_name.to_ascii_lowercase();
  let urls = match key.as_str() {
    "coderankembed" | "coderank" | "nomic-coderankembed" | "code-rank-embed" => {
      ModelUrls::CODERANKEMBED
    }
    _ => {
      return Err(EmbeddingError::Config(format!(
        "unknown model '{model_name}'. supported: coderankembed"
      )));
    }
  };

  let model_path = model_dir.join("CodeRankEmbed-model_quantized.onnx");
  let tokenizer_path = model_dir.join("coderankembed-tokenizer.json");
  ensure_model_exists(&model_path, urls.model_url).await?;
  ensure_tokenizer_exists(&tokenizer_path, urls.tokenizer_url).await?;
  Ok((model_path, tokenizer_path))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn coderank_urls_point_at_hf() {
    assert!(ModelUrls::CODERANKEMBED.model_url.contains("CodeRankEmbed"));
    assert!(ModelUrls::CODERANKEMBED.tokenizer_url.contains("tokenizer.json"));
  }
}
