use std::fmt::Debug;

use pyo3::prelude::*;

use crate::types::py_regex::PyRegExp;
use crate::utils::{HybridRegex, StringOrRegex};

#[derive(Debug, Clone)]
pub struct BindingStringOrRegex(StringOrRegex);

#[cfg(test)]
impl BindingStringOrRegex {
  pub fn new(value: StringOrRegex) -> Self {
    Self(value)
  }
}

impl BindingStringOrRegex {
  pub fn inner(self) -> StringOrRegex {
    self.0
  }
}

impl<'a, 'py> FromPyObject<'a, 'py> for BindingStringOrRegex {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    if let Ok(value) = obj.extract::<String>() {
      return Ok(Self(StringOrRegex::String(value)));
    }

    let regex = PyRegExp::extract(obj)?;
    let hybrid = HybridRegex::try_from(regex).map_err(|err| {
      PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {err}"))
    })?;
    Ok(Self(StringOrRegex::Regex(hybrid)))
  }
}

impl AsRef<StringOrRegex> for BindingStringOrRegex {
  fn as_ref(&self) -> &StringOrRegex {
    &self.0
  }
}

impl TryFrom<BindingStringOrRegex> for HybridRegex {
  type Error = anyhow::Error;

  fn try_from(value: BindingStringOrRegex) -> Result<Self, Self::Error> {
    match value.0 {
      StringOrRegex::String(value) => HybridRegex::new(&value),
      StringOrRegex::Regex(value) => Ok(value),
    }
  }
}

impl From<BindingStringOrRegex> for StringOrRegex {
  fn from(value: BindingStringOrRegex) -> Self {
    value.0
  }
}

pub fn bindingify_string_or_regex_array(items: Vec<BindingStringOrRegex>) -> Vec<StringOrRegex> {
  items.into_iter().map(|item| item.0).collect()
}
