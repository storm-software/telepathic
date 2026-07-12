use serde::{Deserialize, Serialize};

/// Output of the read graph operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadGraphOutput {
  /// The node.
  pub node: String,
}
