use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The input to an export Open Knowledge Format (OKF) operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportOKFInput {
  /// The path to the output location the Open Knowledge Format (OKF) files will be written to.
  pub output_path: PathBuf,
}
