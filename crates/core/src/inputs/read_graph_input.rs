use serde::{Deserialize, Serialize};

/// An input object for reading a node from the source code graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadGraphInput {
  /// The name of the node to read.
  pub name: String,
}
