use serde::{Deserialize, Serialize};

/// Output of a query graph operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryGraphOutput {
  /// The query results.
  pub results: Vec<String>,
}
