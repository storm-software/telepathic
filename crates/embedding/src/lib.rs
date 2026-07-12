//! Neural code embeddings for source retrieval.
//!
//! Default local engine: **CodeRankEmbed** (137M, 768-d) via ONNX Runtime.
//! Optional HTTP: **voyage-code-3** (1024-d). Search uses usearch HNSW; fused
//! scoring blends neural cosine with [`telepathic_semantic`] signals.

mod chunk;
mod config;
mod document;
mod engine;
mod error;
mod index;
mod mock;
mod pool;
mod search;

#[cfg(feature = "onnx")]
mod download;
#[cfg(feature = "onnx")]
mod onnx;
#[cfg(feature = "voyage")]
mod voyage;

pub use chunk::{CHUNK_OVERLAP_TOKENS, CHUNK_SIZE_TOKENS, chunk_document, chunk_text};
pub use config::{EmbeddingConfig, EmbeddingProvider};
pub use document::{
  CodeDocument, body_for_lines, documents_from_source, documents_from_source_lang, format_query,
  metadata_from_definition,
};
pub use engine::EmbeddingEngine;
pub use error::{EmbeddingError, EmbeddingResult};
pub use index::HnswIndex;
pub use mock::MockEmbeddingEngine;
pub use pool::{cosine, l2_normalize, mean_pool};
pub use search::{
  CodeSearcher, CombinedSemanticBoost, MapBoost, NEURAL_WEIGHT, SEMANTIC_WEIGHT, SearchHit,
  SemanticBoost, fuse_scores,
};

#[cfg(feature = "onnx")]
pub use config::OnnxEmbeddingConfig;
#[cfg(feature = "onnx")]
pub use download::{ModelUrls, download_model, ensure_model_exists, ensure_tokenizer_exists};
#[cfg(feature = "onnx")]
pub use onnx::OnnxEmbeddingEngine;
#[cfg(feature = "voyage")]
pub use voyage::VoyageEmbeddingEngine;

/// CodeRankEmbed query instruction (asymmetric retrieval).
pub const CODE_QUERY_PREFIX: &str = "Represent this query for searching relevant code: ";

/// Default local embedding dimensionality (CodeRankEmbed / nomic family).
pub const LOCAL_DIM: usize = 768;

/// Voyage Code 3 dimensionality.
pub const VOYAGE_CODE3_DIM: usize = 1024;
