use serde::{Deserialize, Serialize};

/// The output of the list repositories operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRepositoriesOutput {
  /// The repositories.
  pub repositories: Vec<String>,
}
