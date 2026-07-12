use serde::{Deserialize, Serialize};

/// An input object for requesting a list of projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListProjectsInput {
  /// The id of the repository to list projects for.
  #[serde(rename = "repositoryId")]
  pub repository_id: Option<String>,

  /// All returned projects must depend on the given project.
  #[serde(rename = "dependsOn")]
  pub depends_on: Option<String>,
}
