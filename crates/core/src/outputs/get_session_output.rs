use crate::session::Session;
use serde::{Deserialize, Serialize};

/// Output of session loading.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSessionOutput {
  /// The current session.
  pub session: Session,
}

impl From<Session> for GetSessionOutput {
  fn from(value: Session) -> Self {
    Self { session: value }
  }
}
