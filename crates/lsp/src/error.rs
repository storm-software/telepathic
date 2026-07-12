use thiserror::Error;

/// Errors from the LSP resolve path.
#[derive(Debug, Error)]
pub enum LspError {
  #[error("source code is missing module_qn")]
  MissingModuleQn,
  #[error("source bytes contain interior NUL")]
  InteriorNul,
  #[error("arena allocation failed")]
  ArenaOom,
  #[error("failed to build CBM file result")]
  ConvertFailed,
}
