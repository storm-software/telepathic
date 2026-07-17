use pyo3::prelude::*;
use std::fmt::{self, Display, Formatter};
use telepathic_core::log::LogLevel;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum BindingLogLevel {
  Silent,
  ErrorLevel,
  #[default]
  Warn,
  Info,
  Debug,
}

impl<'a, 'py> FromPyObject<'a, 'py> for BindingLogLevel {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    let value: String = obj.extract()?;
    Self::try_from(value.as_str()).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
  }
}

impl<'py> IntoPyObject<'py> for BindingLogLevel {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    Ok(self.to_string().into_pyobject(py)?.into_any())
  }
}

impl BindingLogLevel {
  fn try_from(value: &str) -> Result<Self, String> {
    match value {
      "silent" => Ok(Self::Silent),
      "error" => Ok(Self::ErrorLevel),
      "warn" => Ok(Self::Warn),
      "info" => Ok(Self::Info),
      "debug" => Ok(Self::Debug),
      _ => Err(format!("Invalid log level: {value}")),
    }
  }
}

impl Display for BindingLogLevel {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Silent => write!(f, "silent"),
      Self::ErrorLevel => write!(f, "error"),
      Self::Warn => write!(f, "warn"),
      Self::Info => write!(f, "info"),
      Self::Debug => write!(f, "debug"),
    }
  }
}

impl From<BindingLogLevel> for LogLevel {
  fn from(value: BindingLogLevel) -> Self {
    match value {
      BindingLogLevel::Silent => Self::Silent,
      BindingLogLevel::ErrorLevel => Self::Error,
      BindingLogLevel::Warn => Self::Warn,
      BindingLogLevel::Info => Self::Info,
      BindingLogLevel::Debug => Self::Debug,
    }
  }
}

impl From<LogLevel> for BindingLogLevel {
  fn from(value: LogLevel) -> Self {
    match value {
      LogLevel::Silent => Self::Silent,
      LogLevel::Error => Self::ErrorLevel,
      LogLevel::Warn => Self::Warn,
      LogLevel::Info => Self::Info,
      LogLevel::Debug => Self::Debug,
    }
  }
}
