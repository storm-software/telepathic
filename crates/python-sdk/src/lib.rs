//! Telepathic binding crate: provides PyO3 (Python) facing helpers to create, start and
//! shutdown a tuned Tokio runtime used by the Telepathic build pipeline.

#![allow(clippy::type_complexity)]
#![allow(clippy::needless_pass_by_value)]

#[cfg(all(
  not(target_family = "wasm"),
  not(feature = "default_global_allocator"),
  not(target_env = "ohos")
))]
#[global_allocator]
static ALLOC: mimalloc_safe::MiMalloc = mimalloc_safe::MiMalloc;

use pyo3::prelude::*;

pub mod binding_engine;
pub mod types;
pub mod utils;
pub mod worker_manager;

use binding_engine::BindingEngine;
use utils::{TraceSubscriberGuard, init_trace_subscriber};

#[pyfunction]
#[pyo3(signature = (blocking_threads=None))]
pub fn create_tokio_runtime(blocking_threads: Option<u32>) {
  let max_blocking_threads = blocking_threads
    .map(|value| value as usize)
    .or_else(|| {
      std::env::var("POWER_PLANT_MAX_BLOCKING_THREADS").ok().and_then(|value| value.parse().ok())
    })
    .unwrap_or(4);

  let mut builder = tokio::runtime::Builder::new_multi_thread();
  builder
    .max_blocking_threads(max_blocking_threads)
    .worker_threads(num_cpus::get_physical() * 3 / 2)
    .enable_all();

  pyo3_async_runtimes::tokio::init(builder);
}

#[pymodule]
mod telepathic_pyo3 {
  #[pymodule_export]
  use super::BindingEngine;

  #[pymodule_export]
  use super::TraceSubscriberGuard;

  #[pymodule_export]
  use super::create_tokio_runtime;

  #[pymodule_export]
  use super::init_trace_subscriber;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
