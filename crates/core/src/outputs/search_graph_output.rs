use serde::{Deserialize, Serialize};

/// A single execution metadata search hit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionSearchHit {
  /// The id of the matching execution.
  #[serde(rename = "executionId")]
  pub execution_id: String,
  /// Relevance score when provided by the search backend.
  pub score: Option<f64>,
  /// Short excerpt from the matched metadata, when available.
  pub snippet: Option<String>,
}

/// Output of a search graph operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchGraphOutput {
  /// The search results.
  pub results: Vec<ExecutionSearchHit>,
}
