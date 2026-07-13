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

pub use context::*;
pub use error::*;
pub use inputs::*;
pub use log::{Log, LogLevel, LogMessage};
pub use normalized_options::*;
pub use options::*;
pub use outputs::*;
pub use repository::*;
pub use session::*;
pub use settings::*;
pub use source_code::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
