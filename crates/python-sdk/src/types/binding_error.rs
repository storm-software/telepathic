#![expect(clippy::missing_debug_implementations)]

use pyo3::prelude::*;

/// Error emitted from native side, it only contains kind and message, no stack trace.
#[derive(Debug, Clone, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct NativeError {
  pub kind: String,
  pub message: String,
}

#[derive(Debug)]
pub enum BindingError {
  PyError(PyErr),
  NativeError(NativeError),
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct BindingErrors {
  #[pyo3(get)]
  pub errors: Vec<NativeError>,
  #[pyo3(get)]
  pub is_binding_errors: bool,
}

#[pymethods]
impl BindingErrors {
  #[new]
  fn new(errors: Vec<NativeError>) -> Self {
    Self { errors, is_binding_errors: true }
  }
}

impl BindingErrors {
  pub fn from_errors(errors: Vec<BindingError>) -> Self {
    Self {
      errors: errors
        .into_iter()
        .map(|error| match error {
          BindingError::PyError(err) => NativeError {
            kind: "PyError".to_string(),
            message: err.to_string(),
          },
          BindingError::NativeError(native_error) => native_error,
        })
        .collect(),
      is_binding_errors: true,
    }
  }
}

pub enum BindingResult<T> {
  Errors(BindingErrors),
  Ok(T),
}
