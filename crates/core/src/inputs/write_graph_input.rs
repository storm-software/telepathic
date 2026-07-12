use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Definition;

/// The input to an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteGraphInput {
  /// The node to write.
  pub node: Definition,
  /// The properties to write.
  pub properties: Option<HashMap<String, Value>>,
}
