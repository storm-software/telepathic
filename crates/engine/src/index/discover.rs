//! Gitignore-aware file discovery.

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use telepathic_tree_sitter::Language;

/// Discover indexable source files under `root` (respects `.gitignore` / `.ignore`).
///
/// Skips hidden paths and files with [`Language::Unknown`] (or no grammar) silently.
pub(super) fn discover_files(root: &Path) -> (Vec<PathBuf>, Vec<String>) {
  let mut files = Vec::new();
  let mut errors = Vec::new();

  let walker = WalkBuilder::new(root)
    .hidden(true)
    .git_ignore(true)
    .git_global(true)
    .git_exclude(true)
    .ignore(true)
    .parents(true)
    .build();

  for entry in walker {
    let entry = match entry {
      Ok(e) => e,
      Err(err) => {
        errors.push(format!("walk error: {err}"));
        continue;
      }
    };

    if !entry.file_type().is_some_and(|ft| ft.is_file()) {
      continue;
    }

    let path = entry.path();
    let lang = Language::from(path);
    if lang == Language::Unknown || lang.language_fn().is_none() {
      continue;
    }

    files.push(path.to_path_buf());
  }

  (files, errors)
}
