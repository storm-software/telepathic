use serde::{Deserialize, Serialize};

/// Output of a get schema operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSchemaOutput {
  /// The schema.
  pub schema: String,
}
