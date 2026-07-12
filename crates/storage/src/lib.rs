//! Execution persistence for the Telepathic runtime.
//!
//! Provides a trait-based interface so storage backends can be swapped without
//! touching the engine or bindings layers.
//!
//! - [`ExecutionStore`] — store, recall, and search [`Execution`] values
//! - [`FsExecutionStore`] — filesystem implementation under the data directory
//! - [`IndexedExecutionStore`] — optional Ladybug graph + vector metadata index

mod error;

#[cfg(feature = "ladybug")]
mod ladybug;

pub use error::StorageError;

#[cfg(feature = "ladybug")]
pub use ladybug::LadybugExecutionIndex;
