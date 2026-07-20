//! Telepathic crate: provides core Telepathic functionalities for creating
//! the React application from the JavaScript/TypeScript source code.

pub mod error;
pub mod sdk;

pub use error::*;
pub use sdk::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
