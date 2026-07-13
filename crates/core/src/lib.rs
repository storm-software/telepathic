//! Telepathic Core crate: provides core types and utilities used across other Telepathic crates.

pub mod context;
pub mod error;
pub mod inputs;
pub mod log;
pub mod normalized_options;
pub mod options;
pub mod outputs;
pub mod repository;
pub mod session;
pub mod settings;
pub mod source_code;

pub use crate::context::*;
pub use crate::error::*;
pub use crate::inputs::*;
pub use crate::log::*;
pub use crate::normalized_options::*;
pub use crate::options::*;
pub use crate::outputs::*;
pub use crate::repository::*;
pub use crate::session::*;
pub use crate::settings::*;
pub use crate::source_code::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
