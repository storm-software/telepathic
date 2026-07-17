use pyo3::prelude::*;
use telepathic_core::outputs::{
  ExecutionSearchHit, ExportOKFOutput, GetSchemaOutput, GetSessionOutput, GetSettingsOutput,
  IndexRepositoryOutput, ListProjectsOutput, ListRepositoriesOutput, QueryGraphOutput,
  ReadGraphOutput, SearchGraphOutput, TraceGraphOutput, WriteGraphOutput,
};

use crate::types::binding_session::BindingSession;
use crate::types::binding_settings::BindingSettings;

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingGetSessionOutput {
  pub session: BindingSession,
}

impl From<GetSessionOutput> for BindingGetSessionOutput {
  fn from(value: GetSessionOutput) -> Self {
    Self { session: BindingSession::from(value.session) }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingGetSettingsOutput {
  pub settings: BindingSettings,
}

impl From<GetSettingsOutput> for BindingGetSettingsOutput {
  fn from(value: GetSettingsOutput) -> Self {
    Self { settings: BindingSettings::from(value.settings) }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingGetSchemaOutput {
  pub schema: String,
}

impl From<GetSchemaOutput> for BindingGetSchemaOutput {
  fn from(value: GetSchemaOutput) -> Self {
    Self { schema: value.schema }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingListRepositoriesOutput {
  pub repositories: Vec<String>,
}

impl From<ListRepositoriesOutput> for BindingListRepositoriesOutput {
  fn from(value: ListRepositoriesOutput) -> Self {
    Self { repositories: value.repositories }
  }
}

#[derive(Clone, PartialEq, Eq, Default, IntoPyObject)]
pub struct BindingIndexRepositoryOutput {
  pub success: bool,
  pub errors: Vec<String>,
}

impl From<IndexRepositoryOutput> for BindingIndexRepositoryOutput {
  fn from(value: IndexRepositoryOutput) -> Self {
    Self { success: value.success, errors: value.errors }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingListProjectsOutput {
  pub projects: Vec<String>,
}

impl From<ListProjectsOutput> for BindingListProjectsOutput {
  fn from(value: ListProjectsOutput) -> Self {
    Self { projects: value.projects }
  }
}

#[derive(Clone, PartialEq, Eq, Default, IntoPyObject)]
pub struct BindingWriteGraphOutput {
  pub success: bool,
  pub errors: Vec<String>,
}

impl From<WriteGraphOutput> for BindingWriteGraphOutput {
  fn from(value: WriteGraphOutput) -> Self {
    Self { success: value.success, errors: value.errors }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingReadGraphOutput {
  pub node: String,
}

impl From<ReadGraphOutput> for BindingReadGraphOutput {
  fn from(value: ReadGraphOutput) -> Self {
    Self { node: value.node }
  }
}

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingQueryGraphOutput {
  pub results: Vec<String>,
}

impl From<QueryGraphOutput> for BindingQueryGraphOutput {
  fn from(value: QueryGraphOutput) -> Self {
    Self { results: value.results }
  }
}

#[derive(Clone, PartialEq, IntoPyObject)]
pub struct BindingExecutionSearchHit {
  pub execution_id: String,
  pub score: Option<f64>,
  pub snippet: Option<String>,
}

#[derive(Clone, PartialEq, IntoPyObject)]
pub struct BindingSearchGraphOutput {
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

#[derive(Clone, PartialEq, Eq, IntoPyObject)]
pub struct BindingTraceGraphOutput {
  pub results: Vec<String>,
}

impl From<TraceGraphOutput> for BindingTraceGraphOutput {
  fn from(value: TraceGraphOutput) -> Self {
    Self { results: value.results }
  }
}

#[derive(Clone, PartialEq, Eq, Default, IntoPyObject)]
pub struct BindingExportOkfOutput {
  pub success: bool,
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
