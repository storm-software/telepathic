use crate::types::binding_session::BindingSession;
use crate::types::binding_settings::BindingSettings;
use napi_derive::napi;
use telepathic_core::outputs::{
  ExecutionSearchHit, ExportOKFOutput, GetSchemaOutput, GetSessionOutput, GetSettingsOutput,
  IndexRepositoryOutput, ListProjectsOutput, ListRepositoriesOutput, QueryGraphOutput,
  ReadGraphOutput, SearchGraphOutput, TraceGraphOutput, WriteGraphOutput,
};

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingGetSessionOutput {
  /// The current session.
  pub session: BindingSession,
}

impl From<GetSessionOutput> for BindingGetSessionOutput {
  fn from(value: GetSessionOutput) -> Self {
    Self { session: BindingSession::from(value.session) }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingGetSettingsOutput {
  /// The loaded settings.
  pub settings: BindingSettings,
}

impl From<GetSettingsOutput> for BindingGetSettingsOutput {
  fn from(value: GetSettingsOutput) -> Self {
    Self { settings: BindingSettings::from(value.settings) }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingGetSchemaOutput {
  /// The schema.
  pub schema: String,
}

impl From<GetSchemaOutput> for BindingGetSchemaOutput {
  fn from(value: GetSchemaOutput) -> Self {
    Self { schema: value.schema }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingListRepositoriesOutput {
  /// The repositories.
  pub repositories: Vec<String>,
}

impl From<ListRepositoriesOutput> for BindingListRepositoriesOutput {
  fn from(value: ListRepositoriesOutput) -> Self {
    Self { repositories: value.repositories }
  }
}

#[derive(Clone, PartialEq, Eq, Default)]
#[napi(object, object_from_js = false)]
pub struct BindingIndexRepositoryOutput {
  /// Whether the index repository operation was successful.
  pub success: bool,
  /// Any errors encountered during the index repository operation.
  pub errors: Vec<String>,
}

impl From<IndexRepositoryOutput> for BindingIndexRepositoryOutput {
  fn from(value: IndexRepositoryOutput) -> Self {
    Self { success: value.success, errors: value.errors }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingListProjectsOutput {
  /// The projects.
  pub projects: Vec<String>,
}

impl From<ListProjectsOutput> for BindingListProjectsOutput {
  fn from(value: ListProjectsOutput) -> Self {
    Self { projects: value.projects }
  }
}

#[derive(Clone, PartialEq, Eq, Default)]
#[napi(object, object_from_js = false)]
pub struct BindingWriteGraphOutput {
  /// Whether the write graph operation was successful.
  pub success: bool,
  /// Any errors encountered during the write graph operation.
  pub errors: Vec<String>,
}

impl From<WriteGraphOutput> for BindingWriteGraphOutput {
  fn from(value: WriteGraphOutput) -> Self {
    Self { success: value.success, errors: value.errors }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingReadGraphOutput {
  /// The node.
  pub node: String,
}

impl From<ReadGraphOutput> for BindingReadGraphOutput {
  fn from(value: ReadGraphOutput) -> Self {
    Self { node: value.node }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingQueryGraphOutput {
  /// The query results.
  pub results: Vec<String>,
}

impl From<QueryGraphOutput> for BindingQueryGraphOutput {
  fn from(value: QueryGraphOutput) -> Self {
    Self { results: value.results }
  }
}

#[derive(Clone, PartialEq)]
#[napi(object, object_from_js = false)]
pub struct BindingExecutionSearchHit {
  /// The id of the matching execution.
  pub execution_id: String,
  /// Relevance score when provided by the search backend.
  pub score: Option<f64>,
  /// Short excerpt from the matched metadata, when available.
  pub snippet: Option<String>,
}

#[derive(Clone, PartialEq)]
#[napi(object, object_from_js = false)]
pub struct BindingSearchGraphOutput {
  /// The search results.
  pub results: Vec<BindingExecutionSearchHit>,
}

impl From<SearchGraphOutput> for BindingSearchGraphOutput {
  fn from(value: SearchGraphOutput) -> Self {
    Self {
      results: value
        .results
        .into_iter()
        .map(|hit| BindingExecutionSearchHit {
          execution_id: hit.execution_id,
          score: hit.score,
          snippet: hit.snippet,
        })
        .collect(),
    }
  }
}

#[derive(Clone, PartialEq, Eq)]
#[napi(object, object_from_js = false)]
pub struct BindingTraceGraphOutput {
  /// The trace results.
  pub results: Vec<String>,
}

impl From<TraceGraphOutput> for BindingTraceGraphOutput {
  fn from(value: TraceGraphOutput) -> Self {
    Self { results: value.results }
  }
}

#[derive(Clone, PartialEq, Eq, Default)]
#[napi(object, object_from_js = false)]
pub struct BindingExportOkfOutput {
  /// Whether the export operation was successful.
  pub success: bool,
  /// Any errors encountered during the export operation.
  pub errors: Vec<String>,
}

impl From<ExportOKFOutput> for BindingExportOkfOutput {
  fn from(value: ExportOKFOutput) -> Self {
    Self { success: value.success, errors: value.errors }
  }
}

impl From<ExecutionSearchHit> for BindingExecutionSearchHit {
  fn from(value: ExecutionSearchHit) -> Self {
    Self {
      execution_id: value.execution_id,
      score: value.score,
      snippet: value.snippet,
    }
  }
}
