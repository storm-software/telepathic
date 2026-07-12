use serde::{Deserialize, Serialize};

/// Output of a trace graph operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceGraphOutput {
  /// The trace results.
  pub results: Vec<String>,
}
