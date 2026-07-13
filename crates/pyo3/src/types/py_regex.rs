use derive_more::Debug;
use pyo3::prelude::*;

use crate::utils::HybridRegex;

#[derive(Debug, Default, Clone)]
pub struct PyRegExp {
  pub source: String,
  pub flags: String,
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyRegExp {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    let py = obj.py();
    let pattern_type = py.import("re")?.getattr("Pattern")?;
    if obj.is_instance(&pattern_type)? {
      let source = obj.getattr("pattern")?.extract::<String>()?;
      let flags = obj.getattr("flags")?.call_method0("value")?.extract::<i32>()?;
      Ok(Self { source, flags: flags_to_string(flags) })
    } else {
      Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Expect a re.Pattern object"))
    }
  }
}

fn flags_to_string(flags: i32) -> String {
  let mut result = String::new();
  if flags & 0x02 != 0 {
    result.push('i');
  }
  if flags & 0x04 != 0 {
    result.push('m');
  }
  if flags & 0x08 != 0 {
    result.push('s');
  }
  if flags & 0x10 != 0 {
    result.push('x');
  }
  if flags & 0x20 != 0 {
    result.push('a');
  }
  if flags & 0x40 != 0 {
    result.push('l');
  }
  if flags & 0x80 != 0 {
    result.push('u');
  }
  result
}

impl TryFrom<PyRegExp> for HybridRegex {
  type Error = anyhow::Error;

  fn try_from(value: PyRegExp) -> Result<Self, Self::Error> {
    HybridRegex::with_flags(&value.source, &value.flags)
  }
}
