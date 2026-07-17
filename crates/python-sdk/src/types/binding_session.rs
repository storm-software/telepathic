use pyo3::prelude::*;
use telepathic_core::session::Session;

use crate::types::{binding_device::BindingDevice, binding_user::BindingUser};

#[derive(Clone, PartialEq, Eq, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingSession {
  pub session_id: String,
  pub started_at: i64,
  pub user: BindingUser,
  pub device: BindingDevice,
}

impl From<Session> for BindingSession {
  fn from(value: Session) -> Self {
    Self {
      session_id: value.session_id.to_string(),
      started_at: value.started_at.timestamp_millis(),
      user: value.user.into(),
      device: value.device.into(),
    }
  }
}
