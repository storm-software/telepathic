use serde::{Deserialize, Serialize};

/// Input for searching stored execution metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchGraphInput {
  /// Free-text query matched against indexed source code graph nodes.
  pub query: Option<String>,
  /// Filter by the user who last modified the node.
  pub last_user_id: Option<String>,
  /// Filter by the name of the node.
  pub name: String,
  /// Filter by the fully qualified name of the node.
  pub qualified_name: String,
  /// Filter by the label of the node.
  pub label: String,
  /// Filter by the file path of the node.
  pub file_path: Option<String>,
  /// Filter by labels; a node matches when any label is present.
  pub labels: Option<Vec<String>>,
  /// Optional embedding vector for semantic similarity search.
  ///
  /// Requires the `ladybug` storage feature and a populated vector index.
  pub embedding: Option<Vec<f32>>,
  /// Maximum number of results to return.
  pub limit: Option<u32>,
}
