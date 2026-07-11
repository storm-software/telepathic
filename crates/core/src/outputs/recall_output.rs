use serde::{Deserialize, Serialize};
use telepathic_models::Execution;

/// Output of a recall operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecallOutput {
  /// The recalled execution.
  pub execution: Execution,
}
