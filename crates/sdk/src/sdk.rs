use std::future::{Future, ready};
use std::sync::{Arc, Mutex};

use derive_more::Debug;
use telepathic_core::{
  Options,
  context::Context,
  inputs::{
    ExportOKFInput, ListProjectsInput, QueryGraphInput, ReadGraphInput, SearchGraphInput,
    TraceGraphInput, WriteGraphInput,
  },
  outputs::{
    ExportOKFOutput, GetSchemaOutput, GetSessionOutput, GetSettingsOutput, ListProjectsOutput,
    ListRepositoriesOutput, QueryGraphOutput, ReadGraphOutput, SearchGraphOutput, TraceGraphOutput,
    WriteGraphOutput,
  },
};
use telepathic_embedding::CodeSearcher;

use crate::{error::SDKError, error::SDKResult};

#[derive(Debug, Clone)]
pub struct Telepathic {
  pub(super) context: Context,
  pub(super) is_closed: bool,
  /// Neural (+ HNSW) index from the latest `index_repository` run.
  #[debug(skip)]
  pub(super) searcher: Arc<Mutex<Option<CodeSearcher>>>,
}

impl Telepathic {
  #[tracing::instrument(skip(options), level = "trace")]
  pub fn new(options: Options) -> SDKResult<Self> {
    let context = Context::new(options);

    Ok(Self { context, is_closed: false, searcher: Arc::new(Mutex::new(None)) })
  }

  pub fn is_closed(&self) -> bool {
    self.is_closed
  }

  /// Borrow the in-memory code searcher, if indexing produced one.
  pub fn searcher(&self) -> &Arc<Mutex<Option<CodeSearcher>>> {
    &self.searcher
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn get_settings(&self) -> SDKResult<GetSettingsOutput> {
    Ok(GetSettingsOutput::from(self.context.settings.clone()))
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn get_session(&self) -> SDKResult<GetSessionOutput> {
    Ok(GetSessionOutput::from(self.context.session.clone()))
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn get_schema(&self) -> SDKResult<GetSchemaOutput> {
    self.create_error_if_closed()?;

    Ok(GetSchemaOutput { schema: String::new() })
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn list_repositories(&self) -> SDKResult<ListRepositoriesOutput> {
    self.create_error_if_closed()?;

    Ok(ListRepositoriesOutput { repositories: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn list_projects(&self, input: ListProjectsInput) -> SDKResult<ListProjectsOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(ListProjectsOutput { projects: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn write_graph(&mut self, input: WriteGraphInput) -> SDKResult<WriteGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(WriteGraphOutput { success: true, errors: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn read_graph(&self, input: ReadGraphInput) -> SDKResult<ReadGraphOutput> {
    self.create_error_if_closed()?;

    Ok(ReadGraphOutput { node: input.name })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn query_graph(&self, input: QueryGraphInput) -> SDKResult<QueryGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(QueryGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn search_graph(&self, input: SearchGraphInput) -> SDKResult<SearchGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(SearchGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn trace_graph(&self, input: TraceGraphInput) -> SDKResult<TraceGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(TraceGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn export_okf(&mut self, input: ExportOKFInput) -> SDKResult<ExportOKFOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(ExportOKFOutput { success: true, errors: vec![] })
  }

  #[must_use = "Future must be awaited to do the actual cleanup work"]
  #[tracing::instrument(skip(self), level = "trace")]
  pub fn close(&mut self) -> impl Future<Output = SDKResult<()>> + Send + 'static {
    self.is_closed = true;
    ready(Ok(()))
  }

  pub(super) fn create_error_if_closed(&self) -> SDKResult<()> {
    if self.is_closed {
      Err(SDKError::Closed)?;
    }

    Ok(())
  }
}

#[cfg(test)]
#[allow(
  clippy::unwrap_used,
  clippy::expect_used,
  reason = "test code — panics are acceptable failures"
)]
mod tests {
  use super::*;
  use std::sync::{Arc, Mutex};
  use telepathic_core::{
    Definition, LogLevel, NormalizedOptions, Repository,
    session::Session,
    settings::{EnvPaths, Mode, Settings},
  };

  fn test_sdk() -> Telepathic {
    let options = Options::default();
    let normalized_options = NormalizedOptions::from(options.clone());
    let context = Context {
      user_options: options,
      options: normalized_options,
      settings: Settings::new(
        Mode::default(),
        LogLevel::default(),
        EnvPaths::default(),
        false,
        "tester".into(),
      ),
      session: Session::default(),
      repository: Repository::default(),
      indexed_sources: Arc::new(Mutex::new(Vec::new())),
    };

    Telepathic { context, is_closed: false, searcher: Arc::new(Mutex::new(None)) }
  }

  #[tokio::test]
  async fn graph_operations_return_expected_defaults() {
    let mut sdk = test_sdk();

    let write_output =
      sdk.write_graph(WriteGraphInput { node: Definition::default(), properties: None }).unwrap();
    assert!(write_output.success);

    let read_output = sdk.read_graph(ReadGraphInput { name: "node".into() }).unwrap();
    assert_eq!(read_output.node, "node");

    let query_output = sdk
      .query_graph(QueryGraphInput { query: "MATCH (n) RETURN n".into(), params: None })
      .unwrap();
    assert!(query_output.results.is_empty());

    let search_output = sdk
      .search_graph(SearchGraphInput {
        query: Some("query".into()),
        last_user_id: None,
        name: String::new(),
        qualified_name: String::new(),
        label: String::new(),
        file_path: None,
        labels: None,
        embedding: None,
        limit: Some(10),
      })
      .unwrap();
    assert!(search_output.results.is_empty());

    let trace_output = sdk
      .trace_graph(TraceGraphInput {
        call_site_name: "call".into(),
        qualified_name: "module.call".into(),
        strategy: "static".into(),
        confidence: 1.0,
      })
      .unwrap();
    assert!(trace_output.results.is_empty());

    let export_output =
      sdk.export_okf(ExportOKFInput { output_path: std::path::PathBuf::from("/tmp/okf") }).unwrap();
    assert!(export_output.success);

    let projects_output =
      sdk.list_projects(ListProjectsInput { repository_id: None, depends_on: None }).unwrap();
    assert!(projects_output.projects.is_empty());

    let repositories_output = sdk.list_repositories().unwrap();
    assert!(repositories_output.repositories.is_empty());

    let schema_output = sdk.get_schema().unwrap();
    assert!(schema_output.schema.is_empty());
  }
}
