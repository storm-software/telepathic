//! Telepathic Language Server Protocol extraction runtime.
//!
//! Wraps vendored CBM per-file LSP resolvers and writes
//! [`telepathic_core::ResolvedCall`] edges into [`telepathic_core::SourceCode`].

mod convert;
mod error;
mod ffi;
mod language;
mod resolve;

pub use error::LspError;
pub use language::LspLanguage;
pub use resolve::resolve;
