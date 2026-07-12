use serde::{Deserialize, Serialize};

/// Output of a write graph operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteGraphOutput {
  /// Whether the write graph operation was successful.
  pub success: bool,
  /// The errors encountered during the write graph operation.
  pub errors: Vec<String>,
}
