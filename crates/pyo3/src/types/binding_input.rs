use std::collections::HashMap;

use pyo3::prelude::*;
use telepathic_core::inputs::{
  ExportOKFInput, ListProjectsInput, QueryGraphInput, ReadGraphInput, SearchGraphInput,
  TraceGraphInput, Value as QueryGraphValue, WriteGraphInput,
};

use crate::types::binding_definition::BindingDefinition;
use crate::utils::json_value::py_dict_to_json_map;

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingWriteGraphInput {
  pub node: BindingDefinition,
  pub properties: Option<HashMap<String, Py<PyAny>>>,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingReadGraphInput {
  pub name: String,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingQueryGraphInput {
  pub query: String,
  pub params: Option<HashMap<String, Py<PyAny>>>,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingSearchGraphInput {
  pub query: Option<String>,
  pub last_user_id: Option<String>,
  pub name: String,
  pub qualified_name: String,
  pub label: String,
  pub file_path: Option<String>,
  pub labels: Option<Vec<String>>,
  pub embedding: Option<Vec<f64>>,
  pub limit: Option<u32>,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingTraceGraphInput {
  pub call_site_name: String,
  pub qualified_name: String,
  pub strategy: String,
  pub confidence: f64,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingExportOkfInput {
  pub output_path: String,
}

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct BindingListProjectsInput {
  pub repository_id: Option<String>,
  pub depends_on: Option<String>,
}

fn json_to_query_value(value: serde_json::Value) -> QueryGraphValue {
  match value {
    serde_json::Value::String(string) => QueryGraphValue::String(string),
    serde_json::Value::Number(number) => {
      QueryGraphValue::Number(number.as_f64().unwrap_or_default())
    }
    serde_json::Value::Bool(boolean) => QueryGraphValue::Boolean(boolean),
    _ => QueryGraphValue::Null,
  }
}

impl BindingWriteGraphInput {
  pub fn into_core(self, py: Python<'_>) -> PyResult<WriteGraphInput> {
    Ok(WriteGraphInput {
      node: self.node.into(),
      properties: self
        .properties
        .map(|params| py_dict_to_json_map(py, params))
        .transpose()?,
    })
  }
}

impl From<BindingReadGraphInput> for ReadGraphInput {
  fn from(value: BindingReadGraphInput) -> Self {
    Self { name: value.name }
  }
}

impl BindingQueryGraphInput {
  pub fn into_core(self, py: Python<'_>) -> PyResult<QueryGraphInput> {
    Ok(QueryGraphInput {
      query: self.query,
      params: self
        .params
        .map(|params| {
          py_dict_to_json_map(py, params).map(|params| {
            params
              .into_iter()
              .map(|(key, value)| (key, json_to_query_value(value)))
              .collect()
          })
        })
        .transpose()?,
    })
  }
}

impl From<BindingSearchGraphInput> for SearchGraphInput {
  fn from(value: BindingSearchGraphInput) -> Self {
    Self {
      query: value.query,
      last_user_id: value.last_user_id,
      name: value.name,
      qualified_name: value.qualified_name,
      label: value.label,
      file_path: value.file_path,
      labels: value.labels,
      embedding: value
        .embedding
        .map(|values| values.into_iter().map(|value| value as f32).collect()),
      limit: value.limit,
    }
  }
}

impl From<BindingTraceGraphInput> for TraceGraphInput {
  fn from(value: BindingTraceGraphInput) -> Self {
    Self {
      call_site_name: value.call_site_name,
      qualified_name: value.qualified_name,
      strategy: value.strategy,
      confidence: value.confidence,
    }
  }
}

impl From<BindingExportOkfInput> for ExportOKFInput {
  fn from(value: BindingExportOkfInput) -> Self {
    Self { output_path: value.output_path.into() }
  }
}

impl From<BindingListProjectsInput> for ListProjectsInput {
  fn from(value: BindingListProjectsInput) -> Self {
    Self { repository_id: value.repository_id, depends_on: value.depends_on }
  }
}
