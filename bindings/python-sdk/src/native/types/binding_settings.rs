use pyo3::prelude::*;
use std::fmt::{self, Display, Formatter};
use telepathic_core::settings::{EnvPaths, Settings};

use crate::types::{
  binding_env_paths::BindingEnvPaths, binding_log_level::BindingLogLevel, binding_mode::BindingMode,
};

#[derive(Clone, PartialEq, Eq, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingSettings {
  pub mode: BindingMode,
  pub default_user: String,
  pub paths: BindingEnvPaths,
  pub log_level: BindingLogLevel,
  pub skip_storage: bool,
}

impl Default for BindingSettings {
  fn default() -> Self {
    Self {
      mode: BindingMode::Production,
      paths: BindingEnvPaths::default(),
      default_user: String::from("default"),
      log_level: BindingLogLevel::Info,
      skip_storage: false,
    }
  }
}

impl Display for BindingSettings {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "mode: {}, default_user: {}, paths: {}, log_level: {}, skip_storage: {}",
      self.mode, self.default_user, self.paths, self.log_level, self.skip_storage
    )
  }
}

impl From<Settings> for BindingSettings {
  fn from(value: Settings) -> Self {
    Self {
      mode: value.mode.into(),
      paths: value.paths.into(),
      log_level: value.log_level.into(),
      default_user: value.default_user,
      skip_storage: value.skip_storage,
    }
  }
}

impl From<BindingSettings> for Settings {
  fn from(value: BindingSettings) -> Self {
    Self {
      mode: value.mode.into(),
      paths: EnvPaths::from(value.paths),
      log_level: value.log_level.into(),
      default_user: value.default_user,
      skip_storage: value.skip_storage,
    }
  }
}

impl From<BindingEnvPaths> for EnvPaths {
  fn from(value: BindingEnvPaths) -> Self {
    Self {
      cache: value.cache.into(),
      config: value.config.into(),
      data: value.data.into(),
      logs: value.logs.into(),
      temp: value.temp.into(),
      downloads: value.downloads.into(),
      executable: value.executable.into(),
    }
  }
}
