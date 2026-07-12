use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Input for executing a [Cypher](https://www.opencypher.org/language-overview) query on the source code graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryGraphInput {
  /// The [Cypher](https://www.opencypher.org/language-overview) query to execute on the source code graph.
  pub query: String,

  /// The params to bind to the query.
  pub params: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
}
