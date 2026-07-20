use std::collections::HashMap;

use crate::types::binding_definition::BindingDefinition;
use napi_derive::napi;
use telepathic_core::inputs::{
  ExportOKFInput, IndexRepositoryInput, ListProjectsInput, QueryGraphInput, ReadGraphInput,
  SearchGraphInput, TraceGraphInput, Value as QueryGraphValue, WriteGraphInput,
};

#[derive(Clone, PartialEq)]
#[napi(object, object_to_js = false)]
pub struct BindingWriteGraphInput {
  /// The node to write.
  pub node: BindingDefinition,
  /// The properties to write.
  pub properties: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[napi(object, object_to_js = false)]
pub struct BindingReadGraphInput {
  /// The name of the node to read.
  pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object)]
pub struct BindingQueryGraphInput {
  /// The Cypher query to execute on the source code graph.
  pub query: String,
  /// The params to bind to the query.
  pub params: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object)]
pub struct BindingSearchGraphInput {
  /// Free-text query matched against indexed source code graph nodes.
  pub query: Option<String>,
  /// Filter by the user who last modified the node.
  pub last_user_id: Option<String>,
  /// Filter by the name of the node.
  pub name: String,
  /// Filter by the fully qualified name of the node.
  pub qualified_name: String,
  /// Filter by the label of the node.
  pub label: String,
  /// Filter by the file path of the node.
  pub file_path: Option<String>,
  /// Filter by labels; a node matches when any label is present.
  pub labels: Option<Vec<String>>,
  /// Optional embedding vector for semantic similarity search.
  pub embedding: Option<Vec<f64>>,
  /// Maximum number of results to return.
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object, object_to_js = false)]
pub struct BindingTraceGraphInput {
  /// The name of the call site to trace.
  pub call_site_name: String,
  /// The fully qualified name of the call site to trace.
  pub qualified_name: String,
  /// The strategy of the call site to trace.
  pub strategy: String,
  /// The confidence of the call site to trace.
  pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object, object_to_js = false)]
pub struct BindingExportOkfInput {
  /// The path to the output location the OKF files will be written to.
  pub output_path: String,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object)]
pub struct BindingIndexRepositoryInput {
  /// The root path of the repository to index. If not provided, the current working directory will be used.
  pub root_path: Option<String>,
  /// Whether to force the repository's files to be indexed even if they have not changed since the last indexing. If not provided, the repository will be indexed only if it has changed since the last indexing.
  pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[napi(object)]
pub struct BindingListProjectsInput {
  /// The id of the repository to list projects for.
  pub repository_id: Option<String>,
  /// All returned projects must depend on the given project.
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

impl From<BindingWriteGraphInput> for WriteGraphInput {
  fn from(value: BindingWriteGraphInput) -> Self {
    Self { node: value.node.into(), properties: value.properties }
  }
}

impl From<BindingReadGraphInput> for ReadGraphInput {
  fn from(value: BindingReadGraphInput) -> Self {
    Self { name: value.name }
  }
}

impl From<BindingQueryGraphInput> for QueryGraphInput {
  fn from(value: BindingQueryGraphInput) -> Self {
    Self {
      query: value.query,
      params: value
        .params
        .map(|params| params.into_iter().map(|(key, value)| (key, json_to_query_value(value))).collect()),
    }
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

impl From<BindingIndexRepositoryInput> for IndexRepositoryInput {
  fn from(value: BindingIndexRepositoryInput) -> Self {
    Self { root_path: value.root_path, force: value.force }
  }
}

impl From<BindingListProjectsInput> for ListProjectsInput {
  fn from(value: BindingListProjectsInput) -> Self {
    Self { repository_id: value.repository_id, depends_on: value.depends_on }
  }
}
