//! Build embeddable documents from tree-sitter / core AST metadata.

use std::collections::BTreeMap;

use telepathic_core::{Definition, SourceCode};
use telepathic_tree_sitter::Language;

use crate::CODE_QUERY_PREFIX;
use crate::chunk::chunk_document;

/// One indexable code unit (definition header ± body chunk).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDocument {
  /// Stable id (`qualified_name#chunk`).
  pub id: String,
  pub qualified_name: String,
  pub name: String,
  pub label: String,
  pub file_path: Option<String>,
  pub start_line: u32,
  pub end_line: u32,
  pub chunk_index: usize,
  /// Full text fed to the embedding engine.
  pub text: String,
  /// Extensible metadata bag for future AST / graph fields.
  pub metadata: BTreeMap<String, String>,
}

impl CodeDocument {
  /// Signature-enriched header from a [`Definition`] (no body).
  pub fn from_definition(def: &Definition, metadata: BTreeMap<String, String>) -> Self {
    let mut doc = Self {
      id: def.qualified_name.clone(),
      qualified_name: def.qualified_name.clone(),
      name: def.name.clone(),
      label: def.label.clone(),
      file_path: def.file_path.clone(),
      start_line: def.start_line,
      end_line: def.end_line,
      chunk_index: 0,
      text: String::new(),
      metadata,
    };
    doc.text = doc.header_text();
    doc
  }

  /// Header text: label, FQN, path, signature, types, decorators, extras.
  pub fn header_text(&self) -> String {
    let mut lines = Vec::new();
    lines.push(format!("[{}] {}", self.label, self.qualified_name));
    if let Some(path) = &self.file_path {
      lines.push(format!("path: {path}"));
    }
    lines.push(format!("name: {}", self.name));
    if let Some(sig) = self.metadata.get("signature") {
      lines.push(format!("signature: {sig}"));
    }
    if let Some(ret) = self.metadata.get("return_type") {
      lines.push(format!("returns: {ret}"));
    }
    if let Some(parent) = self.metadata.get("parent_class") {
      lines.push(format!("parent: {parent}"));
    }
    if let Some(deco) = self.metadata.get("decorators") {
      if !deco.is_empty() {
        lines.push(format!("decorators: {deco}"));
      }
    }
    if let Some(bases) = self.metadata.get("base_classes") {
      if !bases.is_empty() {
        lines.push(format!("bases: {bases}"));
      }
    }
    for (k, v) in &self.metadata {
      if matches!(
        k.as_str(),
        "signature" | "return_type" | "parent_class" | "decorators" | "base_classes"
      ) {
        continue;
      }
      if !v.is_empty() {
        lines.push(format!("{k}: {v}"));
      }
    }
    lines.join("\n")
  }
}

/// Collect definition fields into the extensible metadata map.
pub fn metadata_from_definition(def: &Definition) -> BTreeMap<String, String> {
  let mut meta = BTreeMap::new();
  if let Some(sig) = &def.signature {
    meta.insert("signature".into(), sig.clone());
  }
  if let Some(ret) = &def.return_type {
    meta.insert("return_type".into(), ret.clone());
  }
  if let Some(parent) = &def.parent_class {
    meta.insert("parent_class".into(), parent.clone());
  }
  if !def.decorators.is_empty() {
    meta.insert("decorators".into(), def.decorators.join(", "));
  }
  if !def.base_classes.is_empty() {
    meta.insert("base_classes".into(), def.base_classes.join(", "));
  }
  meta.insert("is_exported".into(), def.is_exported.to_string());
  meta.insert("is_test".into(), def.is_test.to_string());
  meta.insert("is_entry_point".into(), def.is_entry_point.to_string());
  if def.complexity != 0 {
    meta.insert("complexity".into(), def.complexity.to_string());
  }
  if def.lines != 0 {
    meta.insert("lines".into(), def.lines.to_string());
  }
  meta
}

/// Slice source file text by 1-based inclusive line range.
pub fn body_for_lines(file_text: &str, start_line: u32, end_line: u32) -> String {
  if start_line == 0 {
    return String::new();
  }
  file_text
    .lines()
    .skip(start_line.saturating_sub(1) as usize)
    .take((end_line.saturating_sub(start_line).saturating_add(1)) as usize)
    .collect::<Vec<_>>()
    .join("\n")
}

/// Build chunked [`CodeDocument`]s from extracted [`SourceCode`] + file bytes.
///
/// Merges AST metadata into each document; body spans are chunked (256 / 32).
pub fn documents_from_source(source: &SourceCode, file_text: &str) -> Vec<CodeDocument> {
  documents_from_source_lang(source, file_text, None)
}

/// Same as [`documents_from_source`], tagging documents with tree-sitter [`Language`].
pub fn documents_from_source_lang(
  source: &SourceCode,
  file_text: &str,
  language: Option<Language>,
) -> Vec<CodeDocument> {
  let mut out = Vec::new();
  for def in &source.definitions {
    let mut meta = metadata_from_definition(def);
    if let Some(lang) = language {
      meta.insert("language".into(), lang.to_string());
    }
    let base = CodeDocument::from_definition(def, meta);
    let body = body_for_lines(file_text, def.start_line, def.end_line);
    if body.trim().is_empty() {
      out.push(base);
    } else {
      out.extend(chunk_document(&base, &body));
    }
  }
  out
}

/// Format a search query for the active engine.
///
/// - `natural_language`: applies [`CODE_QUERY_PREFIX`] (CodeRankEmbed asymmetric).
/// - `symbol`: embeds the symbol string without the NL instruction prefix.
pub fn format_query(query: &str, natural_language: bool) -> String {
  let q = query.trim();
  if q.is_empty() {
    return String::new();
  }
  if natural_language { format!("{CODE_QUERY_PREFIX}{q}") } else { q.to_owned() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn header_includes_signature() {
    let def = Definition {
      name: "embed".into(),
      qualified_name: "pkg::embed".into(),
      label: "Function".into(),
      file_path: Some("src/lib.rs".into()),
      signature: Some("fn embed(texts: &[&str])".into()),
      return_type: Some("Result<Vec<Vec<f32>>>".into()),
      ..Definition::default()
    };
    let doc = CodeDocument::from_definition(&def, metadata_from_definition(&def));
    assert!(doc.text.contains("signature:"));
    assert!(doc.text.contains("pkg::embed"));
  }

  #[test]
  fn format_query_nl_uses_prefix() {
    let q = format_query("auth middleware", true);
    assert!(q.starts_with(CODE_QUERY_PREFIX));
  }
}
