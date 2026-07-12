use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the user.
pub struct UserInfo {
  /// The name of the user.
  pub name: String,
  /// The display name of the user.
  pub display_name: String,
  /// The email of the user.
  pub email: Option<String>,
  /// The language preferences of the user.
  pub language_preferences: Vec<String>,
}

impl UserInfo {
  pub fn new(
    name: String,
    display_name: String,
    email: Option<String>,
    language_preferences: Vec<String>,
  ) -> Self {
    Self { name, display_name, email, language_preferences }
  }
}

impl Default for UserInfo {
  fn default() -> Self {
    Self {
      name: whoami::username().unwrap_or_else(|_| "Unknown User".to_string()),
      display_name: whoami::realname().unwrap_or_else(|_| "Unknown User".to_string()),
      email: None,
      language_preferences: vec!["en-US".to_string()],
    }
  }
}
