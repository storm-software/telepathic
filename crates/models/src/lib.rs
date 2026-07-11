//! Telepathic Common crate: provides various utilities and helper functions
//! and common types used across other Telepathic crates.

pub mod data;
pub mod data_point;
pub mod document;
pub mod document_chunk;
pub mod embedding;
pub mod entity;
pub mod execution;
pub mod has_datapoint;
pub mod meta;

pub use data::*;
pub use data_point::*;
pub use document::*;
pub use document_chunk::*;
pub use embedding::*;
pub use entity::*;
pub use execution::*;
pub use has_datapoint::*;
pub use meta::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
