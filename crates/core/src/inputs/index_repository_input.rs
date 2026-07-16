use serde::{Deserialize, Serialize};

/// An input object for requesting a repository to be indexed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexRepositoryInput {
  /// The root path of the repository to index. If not provided, the current working directory will be used.
  pub root_path: Option<String>,

  /// Whether to force the repository's files to be indexed even if they have not changed since the last indexing. If not provided, the repository will be indexed only if it has changed since the last indexing.
  pub force: Option<bool>,
}
