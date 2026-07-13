use pyo3::prelude::*;
use telepathic_core::session::UserInfo;

#[derive(Clone, PartialEq, Eq, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingUser {
  pub name: String,
  pub display_name: String,
  pub language_preferences: Vec<String>,
}

impl From<UserInfo> for BindingUser {
  fn from(value: UserInfo) -> Self {
    Self {
      name: value.name,
      display_name: value.display_name,
      language_preferences: value.language_preferences,
    }
  }
}
