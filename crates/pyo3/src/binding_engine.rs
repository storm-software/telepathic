use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

use crate::{
  types::{
    binding_input::{
      BindingExportOkfInput, BindingIndexRepositoryInput, BindingListProjectsInput,
      BindingQueryGraphInput,
      BindingReadGraphInput, BindingSearchGraphInput, BindingTraceGraphInput,
      BindingWriteGraphInput,
    },
    binding_options::BindingOptions,
    binding_output::{
      BindingExportOkfOutput, BindingGetSchemaOutput, BindingGetSessionOutput,
      BindingGetSettingsOutput, BindingIndexRepositoryOutput, BindingListProjectsOutput,
      BindingListRepositoriesOutput, BindingQueryGraphOutput, BindingReadGraphOutput,
      BindingSearchGraphOutput, BindingTraceGraphOutput, BindingWriteGraphOutput,
    },
  },
  utils::map_engine_result,
};
use telepathic_engine::Engine;

use crate::types::binding_error::BindingResult;

fn binding_result_to_py<T>(result: BindingResult<T>) -> PyResult<Py<PyAny>>
where
  T: for<'py> IntoPyObject<'py>,
{
  Python::attach(|py| match result {
    BindingResult::Errors(errors) => Ok(Py::from(errors.into_bound_py_any(py)?)),
    BindingResult::Ok(value) => Ok(Py::from(value.into_bound_py_any(py)?)),
  })
}

#[pyclass]
#[derive(Debug)]
pub struct BindingEngine {
  inner: Engine,
}

#[pymethods]
impl BindingEngine {
  #[new]
  pub fn new(options: BindingOptions) -> PyResult<Self> {
    let inner = Engine::new(options.into_core()).map_err(|err| {
      PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
    })?;

    Ok(Self { inner })
  }

  pub async fn get_settings(&mut self) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.get_settings().map(BindingGetSettingsOutput::from)),
    )
  }

  pub async fn get_session(&mut self) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.get_session().map(BindingGetSessionOutput::from)),
    )
  }

  pub async fn get_schema(&mut self) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.get_schema().map(BindingGetSchemaOutput::from)),
    )
  }

  pub async fn list_repositories(&mut self) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.list_repositories().map(BindingListRepositoriesOutput::from)),
    )
  }

  pub async fn index_repository(
    &mut self,
    input: BindingIndexRepositoryInput,
  ) -> PyResult<Py<PyAny>> {
    binding_result_to_py(map_engine_result(
      self
        .inner
        .index_repository(input.into())
        .await
        .map(BindingIndexRepositoryOutput::from),
    ))
  }

  pub async fn list_projects(&mut self, input: BindingListProjectsInput) -> PyResult<Py<PyAny>> {
    binding_result_to_py(map_engine_result(
      self.inner.list_projects(input.into()).map(BindingListProjectsOutput::from),
    ))
  }

  pub async fn write_graph(&mut self, input: BindingWriteGraphInput) -> PyResult<Py<PyAny>> {
    let core_input = Python::attach(|py| input.into_core(py))?;
    binding_result_to_py(map_engine_result(
      self.inner.write_graph(core_input).map(BindingWriteGraphOutput::from),
    ))
  }

  pub async fn read_graph(&mut self, input: BindingReadGraphInput) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.read_graph(input.into()).map(BindingReadGraphOutput::from)),
    )
  }

  pub async fn query_graph(&mut self, input: BindingQueryGraphInput) -> PyResult<Py<PyAny>> {
    let core_input = Python::attach(|py| input.into_core(py))?;
    binding_result_to_py(map_engine_result(
      self.inner.query_graph(core_input).map(BindingQueryGraphOutput::from),
    ))
  }

  pub async fn search_graph(&mut self, input: BindingSearchGraphInput) -> PyResult<Py<PyAny>> {
    binding_result_to_py(map_engine_result(
      self.inner.search_graph(input.into()).map(BindingSearchGraphOutput::from),
    ))
  }

  pub async fn trace_graph(&mut self, input: BindingTraceGraphInput) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.trace_graph(input.into()).map(BindingTraceGraphOutput::from)),
    )
  }

  pub async fn export_okf(&mut self, input: BindingExportOkfInput) -> PyResult<Py<PyAny>> {
    binding_result_to_py(
      map_engine_result(self.inner.export_okf(input.into()).map(BindingExportOkfOutput::from)),
    )
  }

  pub async fn close(&mut self) -> PyResult<()> {
    self
      .inner
      .close()
      .await
      .map_err(|err| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string()))
  }

  #[getter]
  pub fn is_closed(&self) -> bool {
    self.inner.is_closed()
  }
}
