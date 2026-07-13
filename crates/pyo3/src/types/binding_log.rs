use pyo3::prelude::*;
use telepathic_core::log::{Log, LogMessage};

use crate::types::binding_log_level::BindingLogLevel;

/// Represents a log entry in the Telepathic binding.
#[derive(Clone, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingLog {
  pub message: String,
  pub code: Option<String>,
  pub details: Option<String>,
  pub level: BindingLogLevel,
  pub plugin: Option<String>,
}

impl From<Log> for BindingLog {
  fn from(value: Log) -> Self {
    Self {
      code: value.message.code,
      message: value.message.message,
      details: value.message.details,
      level: BindingLogLevel::from(value.level),
      plugin: value.message.plugin,
    }
  }
}

impl From<BindingLog> for Log {
  fn from(value: BindingLog) -> Self {
    let message = LogMessage {
      message: value.message,
      code: value.code,
      details: value.details,
      plugin: value.plugin,
    };
    let level = BindingLogLevel::into(value.level);
    Self { message, level }
  }
}
