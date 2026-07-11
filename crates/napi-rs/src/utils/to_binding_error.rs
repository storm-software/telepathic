use crate::types::binding_error::{BindingError, NativeError};
use telepathic_core::TelepathicError;

pub fn to_binding_error(err: &dyn TelepathicError) -> BindingError {
  BindingError::NativeError(NativeError { kind: err.kind(), message: err.message() })
}

struct NapiBindingError(napi::Error);

impl TelepathicError for NapiBindingError {
  fn kind(&self) -> String {
    "JsError".to_string()
  }

  fn message(&self) -> String {
    self.0.reason.clone()
  }
}

pub fn to_telepathic_error(err: napi::Error) -> Box<dyn TelepathicError + Send + Sync> {
  Box::new(NapiBindingError(err))
}
