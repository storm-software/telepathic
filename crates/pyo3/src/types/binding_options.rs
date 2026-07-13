use std::sync::Arc;

use pyo3::prelude::*;
use telepathic_core::{Options, log::Logger};

use crate::types::{
  binding_log::BindingLog,
  binding_log_level::BindingLogLevel,
  binding_mode::BindingMode,
  py_callback::{PyCallback, invoke_logger},
};

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingOptions {
  pub mode: Option<BindingMode>,
  pub username: Option<String>,
  pub log_level: Option<BindingLogLevel>,
  pub custom_logger: Option<Py<PyAny>>,
  pub cwd: Option<String>,
  pub repository_root: Option<String>,
}

impl Default for BindingOptions {
  fn default() -> Self {
    Self {
      mode: None,
      username: None,
      log_level: None,
      custom_logger: None,
      cwd: None,
      repository_root: None,
    }
  }
}

impl BindingOptions {
  pub fn into_core(self) -> Options {
    let log_level =
      if self.log_level.is_some() { Some(self.log_level.unwrap().into()) } else { None };

    let custom_logger = self.custom_logger.map(|callback| {
      let callback: PyCallback = Arc::new(callback);
      Logger::new(Arc::new(move |log| {
        let callback = Arc::clone(&callback);
        Box::pin(async move { invoke_logger(&callback, BindingLog::from(log)).await })
      }))
    });

    Options {
      mode: self.mode.map(Into::into),
      username: self.username,
      log_level,
      custom_logger,
      cwd: self.cwd,
      repository_root: self.repository_root,
    }
  }
}
