use thiserror::Error;

/// Errors from embedding generation, indexing, or search.
#[derive(Error, Debug)]
pub enum EmbeddingError {
  #[error("model load error: {0}")]
  ModelLoad(String),

  #[error("tokenizer error: {0}")]
  Tokenizer(String),

  #[error("inference error: {0}")]
  Inference(String),

  #[error("configuration error: {0}")]
  Config(String),

  #[error("index error: {0}")]
  Index(String),

  #[error("HTTP error: {0}")]
  Http(String),

  #[error("API error: {0}")]
  Api(String),

  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("provider not enabled: {0}")]
  NotEnabled(String),
}

/// Result alias for embedding operations.
pub type EmbeddingResult<T> = Result<T, EmbeddingError>;
