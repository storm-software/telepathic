//! Repository indexing: discover → extract (+ LSP) → embed.

mod discover;
mod extract;
mod lsp_lang;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use camino::Utf8PathBuf;
use telepathic_core::SourceCode;
use telepathic_embedding::{CodeSearcher, EmbeddingConfig, documents_from_source_lang};

use crate::EngineResult;

pub(crate) struct IndexOutcome {
  pub sources: Vec<SourceCode>,
  pub searcher: Option<CodeSearcher>,
  pub errors: Vec<String>,
  pub walk_started: bool,
}

/// Run full index: gitignore-aware discover, rayon extract+LSP, async embed.
pub(crate) async fn index_repository(
  root: Utf8PathBuf,
  project: String,
) -> EngineResult<IndexOutcome> {
  let root_path = PathBuf::from(root.as_str());
  if !root_path.is_dir() {
    return Ok(IndexOutcome {
      sources: Vec::new(),
      searcher: None,
      errors: vec![format!("repository root is not a directory: {root}")],
      walk_started: false,
    });
  }

  let (files, mut errors) = discover::discover_files(&root_path);
  let walk_started = true;

  let extract_results = extract::extract_files_parallel(&root_path, &project, &files);

  let mut sources = Vec::with_capacity(extract_results.len());
  let mut docs = Vec::new();
  for result in extract_results {
    if let Some(err) = result.error {
      errors.push(err);
    }
    if let Some(source) = result.source {
      docs.extend(documents_from_source_lang(&source, &result.text, Some(result.language)));
      sources.push(source);
    }
  }

  let mut searcher = None;
  match EmbeddingConfig::from_env().create_engine().await {
    Ok(engine) => match CodeSearcher::new(engine) {
      Ok(mut cs) => {
        if let Err(err) = cs.index_documents(&docs).await {
          errors.push(format!("embed index failed: {err}"));
        } else {
          searcher = Some(cs);
        }
      }
      Err(err) => errors.push(format!("code searcher init failed: {err}")),
    },
    Err(err) => errors.push(format!("embedding engine init failed: {err}")),
  }

  Ok(IndexOutcome { sources, searcher, errors, walk_started })
}

/// Replace in-memory index state (sources + searcher).
pub(crate) fn store_outcome(
  indexed_sources: &Arc<Mutex<Vec<SourceCode>>>,
  searcher: &Arc<Mutex<Option<CodeSearcher>>>,
  outcome: IndexOutcome,
) {
  if let Ok(mut guard) = indexed_sources.lock() {
    *guard = outcome.sources;
  }
  if let Ok(mut guard) = searcher.lock() {
    *guard = outcome.searcher;
  }
}
