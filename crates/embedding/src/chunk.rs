//! Chunk size defaults: ~256 tokens, 32 overlap (grilling decision).

use crate::document::CodeDocument;

/// Target tokens per body chunk.
pub const CHUNK_SIZE_TOKENS: usize = 256;

/// Overlap between consecutive chunks (tokens).
pub const CHUNK_OVERLAP_TOKENS: usize = 32;

/// Approximate chars per token for code (conservative).
const CHARS_PER_TOKEN: usize = 4;

/// Split `text` into overlapping chunks sized by approximate token count.
pub fn chunk_text(text: &str, chunk_tokens: usize, overlap_tokens: usize) -> Vec<String> {
  let chunk_chars = chunk_tokens.saturating_mul(CHARS_PER_TOKEN).max(1);
  let overlap_chars = overlap_tokens.saturating_mul(CHARS_PER_TOKEN);
  let step = chunk_chars.saturating_sub(overlap_chars).max(1);

  let trimmed = text.trim();
  if trimmed.is_empty() {
    return Vec::new();
  }
  if trimmed.len() <= chunk_chars {
    return vec![trimmed.to_owned()];
  }

  let mut out = Vec::new();
  let bytes = trimmed.as_bytes();
  let mut start = 0_usize;
  while start < bytes.len() {
    let mut end = (start + chunk_chars).min(bytes.len());
    // Prefer breaking on newline when near the end of the window.
    if end < bytes.len() {
      if let Some(rel) = trimmed[start..end].rfind('\n') {
        let candidate = start + rel + 1;
        if candidate > start {
          end = candidate;
        }
      }
    }
    let slice = std::str::from_utf8(&bytes[start..end])
      .unwrap_or("")
      .trim();
    if !slice.is_empty() {
      out.push(slice.to_owned());
    }
    if end >= bytes.len() {
      break;
    }
    start += step;
    if start >= bytes.len() {
      break;
    }
  }
  out
}

/// Expand one definition document into chunked documents (header + each body chunk).
pub fn chunk_document(base: &CodeDocument, body: &str) -> Vec<CodeDocument> {
  let chunks = chunk_text(body, CHUNK_SIZE_TOKENS, CHUNK_OVERLAP_TOKENS);
  if chunks.is_empty() {
    return vec![base.clone()];
  }

  chunks
    .into_iter()
    .enumerate()
    .map(|(idx, chunk)| {
      let mut doc = base.clone();
      doc.chunk_index = idx;
      doc.id = format!("{}#{}", base.qualified_name, idx);
      if chunk.is_empty() {
        doc.text = base.header_text();
      } else {
        doc.text = format!("{}\n\n{}", base.header_text(), chunk);
      }
      doc
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn short_text_single_chunk() {
    let chunks = chunk_text("fn hello() {}", 256, 32);
    assert_eq!(chunks.len(), 1);
  }

  #[test]
  fn long_text_overlaps() {
    let body = "line\n".repeat(400);
    let chunks = chunk_text(&body, 256, 32);
    assert!(chunks.len() > 1);
  }
}
