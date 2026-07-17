use std::collections::HashMap;

use pyo3::prelude::*;

pub fn py_dict_to_json_map(
  py: Python<'_>,
  params: HashMap<String, Py<PyAny>>,
) -> PyResult<HashMap<String, serde_json::Value>> {
  params
    .into_iter()
    .map(|(key, value)| py_any_to_json(py, &value).map(|json| (key, json)))
    .collect()
}

fn py_any_to_json(py: Python<'_>, value: &Py<PyAny>) -> PyResult<serde_json::Value> {
  let bound = value.bind(py);
  if bound.is_none() {
    return Ok(serde_json::Value::Null);
  }
  if let Ok(value) = bound.extract::<bool>() {
    return Ok(serde_json::Value::Bool(value));
  }
  if let Ok(value) = bound.extract::<i64>() {
    return Ok(serde_json::Value::Number(value.into()));
  }
  if let Ok(value) = bound.extract::<f64>() {
    if let Some(number) = serde_json::Number::from_f64(value) {
      return Ok(serde_json::Value::Number(number));
    }
  }
  if let Ok(value) = bound.extract::<String>() {
    return Ok(serde_json::Value::String(value));
  }

  Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
    "Unsupported JSON value type in binding params",
  ))
}
