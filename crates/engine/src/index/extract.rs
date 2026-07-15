//! Parallel per-file extract + optional LSP resolve.

use std::path::{Path, PathBuf};

use rayon::prelude::*;
use telepathic_core::SourceCode;
use telepathic_lsp::resolve;
use telepathic_tree_sitter::{Language, extract_on_thread};

use super::lsp_lang::to_lsp_language;

pub(super) struct FileExtractResult {
  pub language: Language,
  pub text: String,
  pub source: Option<SourceCode>,
  pub error: Option<String>,
}

/// Extract (+ LSP when available) each file in parallel via rayon.
pub(super) fn extract_files_parallel(
  root: &Path,
  project: &str,
  files: &[PathBuf],
) -> Vec<FileExtractResult> {
  files
    .par_iter()
    .map(|path| extract_one(root, project, path))
    .collect()
}

fn extract_one(root: &Path, project: &str, path: &Path) -> FileExtractResult {
  let language = Language::from(path);
  let rel_path = path.strip_prefix(root).unwrap_or(path);
  let rel_str = rel_path.to_string_lossy();

  let bytes = match std::fs::read(path) {
    Ok(b) => b,
    Err(err) => {
      return FileExtractResult {
        language,
        text: String::new(),
        source: None,
        error: Some(format!("{rel_str}: read failed: {err}")),
      };
    }
  };

  let text = String::from_utf8_lossy(&bytes).into_owned();

  let (mut source, tree) = match extract_on_thread(language, &bytes, project, &rel_str) {
    Ok(pair) => pair,
    Err(err) => {
      return FileExtractResult {
        language,
        text,
        source: None,
        error: Some(format!("{rel_str}: extract failed: {err}")),
      };
    }
  };

  if source.has_error {
    let msg = source.error_msg.clone().unwrap_or_else(|| "extract error".into());
    return FileExtractResult {
      language,
      text,
      source: Some(source),
      error: Some(format!("{rel_str}: {msg}")),
    };
  }

  if let (Some(lsp_lang), Some(tree)) = (to_lsp_language(language), tree.as_ref()) {
    if let Err(err) = resolve(lsp_lang, &bytes, tree, &mut source) {
      return FileExtractResult {
        language,
        text,
        source: Some(source),
        error: Some(format!("{rel_str}: lsp resolve failed: {err}")),
      };
    }
  }

  FileExtractResult { language, text, source: Some(source), error: None }
}
