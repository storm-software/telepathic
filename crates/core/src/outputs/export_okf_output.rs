use serde::{Deserialize, Serialize};

/// Output of an export Open Knowledge Format (OKF) operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportOKFOutput {
  /// Whether the export operation was successful.
  pub success: bool,
  /// The errors encountered during the export operation.
  pub errors: Vec<String>,
}
