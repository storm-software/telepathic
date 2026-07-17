use std::sync::Arc;

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::into_future;

use crate::types::binding_log::BindingLog;
use crate::utils::to_telepathic_error;

pub type PyCallback = Arc<Py<PyAny>>;

pub async fn invoke_logger(
  callback: &PyCallback,
  log: BindingLog,
) -> telepathic_core::TelepathicResult<()> {
  let maybe_future = Python::attach(|py| -> PyResult<Option<_>> {
    let result = callback.bind(py).call1((log,))?;
    if result.is_instance_of::<pyo3::coroutine::Coroutine>() {
      Ok(Some(into_future(result)?))
    } else {
      Ok(None)
    }
  })
  .map_err(to_telepathic_error)?;

  if let Some(future) = maybe_future {
    future.await.map_err(to_telepathic_error)?;
  }

  Ok(())
}
