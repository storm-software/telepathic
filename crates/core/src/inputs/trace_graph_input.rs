use serde::{Deserialize, Serialize};

/// Input for tracing a call site in the source code graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TraceGraphInput {
  /// The name of the call site to trace.
  pub call_site_name: String,
  /// The fully qualified name of the call site to trace.
  pub qualified_name: String,
  /// The strategy of the call site to trace.
  pub strategy: String,
  /// The confidence of the call site to trace.
  pub confidence: f64,
}
