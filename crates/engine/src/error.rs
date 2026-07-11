//! Error types for Telepathic engine.

use telepathic_core::TelepathicError;

/// Errors that can occur during Telepathic engine operations.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
  /// Engine is closed.
  #[error("Engine is closed")]
  EngineClosed,
  /// Engine is not initialized.
  #[error("Engine is not initialized")]
  EngineNotInitialized,
  /// Storage error.
  #[error("Storage error: {0}")]
  StorageError(String),
  /// Resource not found.
  #[error("resource not found")]
  NotFound,
  /// Operation cancelled.
  #[error("operation cancelled")]
  Cancelled,
  /// Discover failed.
  #[error("discover failed (rc={0})")]
  DiscoverFailed(i32),
  /// Extraction failed.
  #[error("extraction failed (rc={0})")]
  ExtractionFailed(i32),
  /// Dump failed.
  #[error("dump failed (rc={0})")]
  DumpFailed(i32),
  /// Artifact export failed.
  #[error("artifact export failed")]
  ArtifactExportFailed,
  /// Invalid project name.
  #[error("invalid project name")]
  InvalidProjectName,
  /// Other error.
  #[error("{0}")]
  Other(String),
}

impl TelepathicError for EngineError {
  fn kind(&self) -> String {
    "Engine".to_string()
  }

  fn message(&self) -> String {
    self.to_string()
  }
}

/// Result type for Telepathic engine operations.
pub type EngineResult<T> = Result<T, EngineError>;
