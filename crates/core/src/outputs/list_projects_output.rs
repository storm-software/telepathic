use serde::{Deserialize, Serialize};

/// The output of the list projects operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListProjectsOutput {
  /// The projects.
  pub projects: Vec<String>,
}
