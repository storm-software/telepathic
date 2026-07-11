use serde::{Deserialize, Serialize};
use telepathic_models::Execution;

/// The input to an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreInput {
  /// The execution that produced the input.
  pub execution: Execution,
}
