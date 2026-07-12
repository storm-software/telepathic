use serde::{Deserialize, Serialize};

/// Output of the index repository operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexRepositoryOutput {
  /// Whether the index repository operation was successful.
  pub success: bool,
  /// Any errors encountered during the index repository operation.
  pub errors: Vec<String>,
}
