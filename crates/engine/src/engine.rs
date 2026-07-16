use std::future::{Future, ready};
use std::sync::{Arc, Mutex};

use derive_more::Debug;
use telepathic_core::{
  Options, Repository,
  context::Context,
  inputs::{
    ExportOKFInput, IndexRepositoryInput, ListProjectsInput, QueryGraphInput, ReadGraphInput,
    SearchGraphInput, TraceGraphInput, WriteGraphInput,
  },
  outputs::{
    ExportOKFOutput, GetSchemaOutput, GetSessionOutput, GetSettingsOutput, IndexRepositoryOutput,
    ListProjectsOutput, ListRepositoriesOutput, QueryGraphOutput, ReadGraphOutput,
    SearchGraphOutput, TraceGraphOutput, WriteGraphOutput,
  },
};
use telepathic_embedding::CodeSearcher;

use crate::{EngineError, EngineResult, index};

#[derive(Debug, Clone)]
pub struct Engine {
  pub(super) context: Context,
  pub(super) is_closed: bool,
  /// Neural (+ HNSW) index from the latest `index_repository` run.
  #[debug(skip)]
  pub(super) searcher: Arc<Mutex<Option<CodeSearcher>>>,
}

impl Engine {
  #[tracing::instrument(skip(options), level = "trace")]
  pub fn new(options: Options) -> EngineResult<Self> {
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
  pub fn get_settings(&self) -> EngineResult<GetSettingsOutput> {
    Ok(GetSettingsOutput::from(self.context.settings.clone()))
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn get_session(&self) -> EngineResult<GetSessionOutput> {
    Ok(GetSessionOutput::from(self.context.session.clone()))
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn get_schema(&self) -> EngineResult<GetSchemaOutput> {
    self.create_error_if_closed()?;

    Ok(GetSchemaOutput { schema: String::new() })
  }

  #[tracing::instrument(skip_all, level = "trace")]
  pub fn list_repositories(&self) -> EngineResult<ListRepositoriesOutput> {
    self.create_error_if_closed()?;

    Ok(ListRepositoriesOutput { repositories: vec![] })
  }

  /// Discover, extract (+ LSP), and embed the repository into in-memory state.
  ///
  /// Returns a `'static` future (same pattern as [`Self::close`]) so napi can
  /// `spawn_future` without holding `&mut self` across await.
  #[must_use = "Future must be awaited to run indexing"]
  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn index_repository(
    &mut self,
    input: IndexRepositoryInput,
  ) -> impl Future<Output = EngineResult<IndexRepositoryOutput>> + Send + 'static {
    let IndexRepositoryInput { root_path, force } = input;
    let is_closed = self.is_closed;
    let force = force.unwrap_or(false);
    let (root, project) = match root_path {
      Some(path) => {
        let repo = Repository::from(camino::Utf8PathBuf::from(path));
        (repo.root_path, repo.name)
      }
      None => {
        (self.context.repository.root_path.clone(), self.context.repository.name.clone())
      }
    };
    let indexed_sources = Arc::clone(&self.context.indexed_sources);
    let searcher = Arc::clone(&self.searcher);

    let skip = !force
      && indexed_sources.lock().map(|guard| !guard.is_empty()).unwrap_or(false);

    if !skip {
      // Replace semantics: clear before the async work starts.
      if let Ok(mut guard) = indexed_sources.lock() {
        guard.clear();
      }
      if let Ok(mut guard) = searcher.lock() {
        *guard = None;
      }
    }

    async move {
      if is_closed {
        return Err(EngineError::EngineClosed);
      }

      if skip {
        return Ok(IndexRepositoryOutput { success: true, errors: vec![] });
      }

      let outcome = index::index_repository(root, project).await?;
      let success = outcome.walk_started;
      let errors = outcome.errors.clone();
      index::store_outcome(&indexed_sources, &searcher, outcome);
      Ok(IndexRepositoryOutput { success, errors })
    }
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn list_projects(&self, input: ListProjectsInput) -> EngineResult<ListProjectsOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(ListProjectsOutput { projects: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn write_graph(&mut self, input: WriteGraphInput) -> EngineResult<WriteGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(WriteGraphOutput { success: true, errors: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn read_graph(&self, input: ReadGraphInput) -> EngineResult<ReadGraphOutput> {
    self.create_error_if_closed()?;

    Ok(ReadGraphOutput { node: input.name })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn query_graph(&self, input: QueryGraphInput) -> EngineResult<QueryGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(QueryGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn search_graph(&self, input: SearchGraphInput) -> EngineResult<SearchGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(SearchGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn trace_graph(&self, input: TraceGraphInput) -> EngineResult<TraceGraphOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(TraceGraphOutput { results: vec![] })
  }

  #[tracing::instrument(skip(self, input), level = "trace")]
  pub fn export_okf(&mut self, input: ExportOKFInput) -> EngineResult<ExportOKFOutput> {
    self.create_error_if_closed()?;
    let _ = input;

    Ok(ExportOKFOutput { success: true, errors: vec![] })
  }

  #[must_use = "Future must be awaited to do the actual cleanup work"]
  #[tracing::instrument(skip(self), level = "trace")]
  pub fn close(&mut self) -> impl Future<Output = EngineResult<()>> + Send + 'static {
    self.is_closed = true;
    ready(Ok(()))
  }

  pub(super) fn create_error_if_closed(&self) -> EngineResult<()> {
    if self.is_closed {
      Err(EngineError::EngineClosed)?;
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
    Definition, LogLevel, NormalizedOptions, Repository, session::Session,
    settings::{EnvPaths, Mode, Settings},
  };

  fn test_engine() -> Engine {
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

    Engine { context, is_closed: false, searcher: Arc::new(Mutex::new(None)) }
  }

  #[tokio::test]
  async fn index_repository_indexes_temp_project() {
    unsafe {
      std::env::set_var("MOCK_EMBEDDING", "1");
      std::env::set_var("EMBEDDING_PROVIDER", "mock");
    }

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("main.rs");
    std::fs::write(&src, "fn hello() {}\n").unwrap();

    let options = Options::default();
    let normalized_options = NormalizedOptions::from(options.clone());
    let root = camino::Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).unwrap();
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
      repository: Repository {
        root_path: root,
        name: "fixture".into(),
      },
      indexed_sources: Arc::new(Mutex::new(Vec::new())),
    };
    let mut engine =
      Engine { context, is_closed: false, searcher: Arc::new(Mutex::new(None)) };

    let index_output = engine
      .index_repository(IndexRepositoryInput { root_path: None, force: None })
      .await
      .unwrap();
    assert!(index_output.success);

    let sources = engine.context.indexed_sources.lock().unwrap();
    assert!(!sources.is_empty(), "expected at least one extracted file");
  }

  #[tokio::test]
  async fn graph_operations_return_expected_defaults() {
    let mut engine = test_engine();

    let write_output = engine
      .write_graph(WriteGraphInput { node: Definition::default(), properties: None })
      .unwrap();
    assert!(write_output.success);

    let read_output = engine.read_graph(ReadGraphInput { name: "node".into() }).unwrap();
    assert_eq!(read_output.node, "node");

    let query_output = engine
      .query_graph(QueryGraphInput { query: "MATCH (n) RETURN n".into(), params: None })
      .unwrap();
    assert!(query_output.results.is_empty());

    let search_output = engine
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

    let trace_output = engine
      .trace_graph(TraceGraphInput {
        call_site_name: "call".into(),
        qualified_name: "module.call".into(),
        strategy: "static".into(),
        confidence: 1.0,
      })
      .unwrap();
    assert!(trace_output.results.is_empty());

    let export_output = engine
      .export_okf(ExportOKFInput { output_path: std::path::PathBuf::from("/tmp/okf") })
      .unwrap();
    assert!(export_output.success);

    let projects_output = engine
      .list_projects(ListProjectsInput { repository_id: None, depends_on: None })
      .unwrap();
    assert!(projects_output.projects.is_empty());

    let repositories_output = engine.list_repositories().unwrap();
    assert!(repositories_output.repositories.is_empty());

    let schema_output = engine.get_schema().unwrap();
    assert!(schema_output.schema.is_empty());
  }
}
