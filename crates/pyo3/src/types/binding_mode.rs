use std::fmt::{self, Display, Formatter};
use pyo3::prelude::*;
use telepathic_core::settings::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingMode {
  Development,
  Production,
  Test,
}

impl<'a, 'py> FromPyObject<'a, 'py> for BindingMode {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    let value: String = obj.extract()?;
    Self::try_from(value.as_str()).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
  }
}

impl<'py> IntoPyObject<'py> for BindingMode {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    Ok(self.to_string().into_pyobject(py)?.into_any())
  }
}

impl BindingMode {
  fn try_from(value: &str) -> Result<Self, String> {
    match value {
      "development" => Ok(Self::Development),
      "production" => Ok(Self::Production),
      "test" => Ok(Self::Test),
      _ => Err(format!("Invalid app mode: {value}")),
    }
  }
}

impl Display for BindingMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Development => write!(f, "development"),
      Self::Production => write!(f, "production"),
      Self::Test => write!(f, "test"),
    }
  }
}

impl From<BindingMode> for Mode {
  fn from(value: BindingMode) -> Self {
    match value {
      BindingMode::Development => Self::Development,
      BindingMode::Production => Self::Production,
      BindingMode::Test => Self::Test,
    }
  }
}

impl From<Mode> for BindingMode {
  fn from(value: Mode) -> Self {
    match value {
      Mode::Development => Self::Development,
      Mode::Production => Self::Production,
      Mode::Test => Self::Test,
    }
  }
}
