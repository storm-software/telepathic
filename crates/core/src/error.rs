//! Error types for Telepathic engine.

pub trait TelepathicError {
  fn kind(&self) -> String;
  fn message(&self) -> String;
}

pub type TelepathicResult<T> = Result<T, Box<dyn TelepathicError + Send + Sync>>;
