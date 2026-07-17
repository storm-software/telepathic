use pyo3::prelude::*;
use telepathic_core::TelepathicError;
use telepathic_engine::EngineError;

use crate::types::binding_error::{BindingError, BindingErrors, BindingResult, NativeError};

pub fn to_binding_error(err: &dyn TelepathicError) -> BindingError {
  BindingError::NativeError(NativeError { kind: err.kind(), message: err.message() })
}

pub fn to_binding_errors(err: &dyn TelepathicError) -> BindingErrors {
  BindingErrors::from_errors(vec![to_binding_error(err)])
}

pub fn map_engine_result<T>(result: Result<T, EngineError>) -> BindingResult<T> {
  match result {
    Ok(value) => BindingResult::Ok(value),
    Err(err) => BindingResult::Errors(to_binding_errors(&err)),
  }
}

struct PyBindingError(PyErr);

impl TelepathicError for PyBindingError {
  fn kind(&self) -> String {
    "PyError".to_string()
  }

  fn message(&self) -> String {
    self.0.to_string()
  }
}

pub fn to_telepathic_error(err: PyErr) -> Box<dyn TelepathicError + Send + Sync> {
  Box::new(PyBindingError(err))
}
