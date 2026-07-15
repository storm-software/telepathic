//! Indexing pipeline orchestrator
//!
//! Coordinates multi-pass repository indexing:
//!   1. Discover files
//!   2. Build structure (Project/Folder/Package/File nodes)
//!   3. Extract definitions + resolve imports/calls/usages/semantic edges
//!   4. Post-passes (tests, communities, config links, git history)
//!   5. Dump graph buffer to SQLite
//!
//! Individual pass implementations plug in through [`GraphIndexer`] and
//! [`FileDiscoverer`] traits so the C extraction/LSP backends can be wired
//! via FFI without rewriting the orchestration layer.

mod error;
mod fqn;
mod lang_spec;
mod lang_spec_gen;
mod languages;
mod parser;
mod types;
mod walk;

pub use error::TreeSitterError;
pub use lang_spec::{LangSpec, class_label_for_kind, kind_in, lang_spec, manifest_lang_spec};
pub use lang_spec_gen::modules_for;
pub use parser::{LanguageParser, configure_parser, extract_on_thread, parse_on_thread, parser_for};
pub use types::Language;

pub(crate) use fqn::{compute_fqn, module_dir_fqn};
pub(crate) use walk::extract_from_tree;
