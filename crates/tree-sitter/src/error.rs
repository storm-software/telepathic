use crate::Language;

/// Errors that can occur while loading a grammar into a parser.
#[derive(Debug, thiserror::Error)]
pub enum TreeSitterError {
  /// No compiled grammar is available for this [`crate::Language`] (e.g. [`crate::Language::Unknown`]).
  #[error("no tree-sitter grammar available for language {0:?}")]
  Unavailable(Language),
  /// The grammar ABI is incompatible with the linked `tree-sitter` crate.
  #[error(transparent)]
  Incompatible(#[from] tree_sitter::LanguageError),
}

impl PartialEq for TreeSitterError {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Unavailable(a), Self::Unavailable(b)) => a == b,
      (Self::Incompatible(a), Self::Incompatible(b)) => a == b,
      _ => false,
    }
  }
}

impl Eq for TreeSitterError {}
