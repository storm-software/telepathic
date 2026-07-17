use std::any::Any;

use pyo3::prelude::*;
use telepathic_tracing::try_init_tracing;

#[pyclass(unsendable)]
#[derive(Debug)]
pub struct TraceSubscriberGuard {
  guard: Option<Box<dyn Any + Send>>,
}

#[pymethods]
impl TraceSubscriberGuard {
  pub fn close(&mut self) {
    self.guard.take();
  }
}

#[pyfunction]
pub fn init_trace_subscriber() -> Option<TraceSubscriberGuard> {
  let maybe_guard = try_init_tracing();
  let guard = maybe_guard?;
  Some(TraceSubscriberGuard { guard: Some(guard) })
}
