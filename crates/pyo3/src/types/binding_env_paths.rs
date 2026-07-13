use pyo3::prelude::*;
use std::fmt::{self, Display, Formatter};
use telepathic_core::settings::EnvPaths;

#[derive(Debug, Clone, PartialEq, Eq, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingEnvPaths {
  pub cache: String,
  pub config: String,
  pub data: String,
  pub logs: String,
  pub temp: String,
  pub downloads: String,
  pub executable: String,
}

impl Display for BindingEnvPaths {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "cache: {}, config: {}, data: {}, logs: {}, temp: {}, downloads: {}, executable: {}",
      self.cache, self.config, self.data, self.logs, self.temp, self.downloads, self.executable
    )
  }
}

impl From<EnvPaths> for BindingEnvPaths {
  fn from(value: EnvPaths) -> Self {
    Self {
      cache: value.cache.to_string(),
      config: value.config.to_string(),
      data: value.data.to_string(),
      logs: value.logs.to_string(),
      temp: value.temp.to_string(),
      downloads: value.downloads.to_string(),
      executable: value.executable.to_string(),
    }
  }
}

impl Default for BindingEnvPaths {
  fn default() -> Self {
    Self {
      cache: EnvPaths::default().cache.to_string(),
      config: EnvPaths::default().config.to_string(),
      data: EnvPaths::default().data.to_string(),
      logs: EnvPaths::default().logs.to_string(),
      temp: EnvPaths::default().temp.to_string(),
      downloads: EnvPaths::default().downloads.to_string(),
      executable: EnvPaths::default().executable.to_string(),
    }
  }
}
