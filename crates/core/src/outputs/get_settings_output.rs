use serde::{Deserialize, Serialize};

use crate::settings::Settings;

/// Output of a get settings operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSettingsOutput {
  /// The settings.
  pub settings: Settings,
}

impl From<Settings> for GetSettingsOutput {
  fn from(value: Settings) -> Self {
    Self { settings: value }
  }
}
