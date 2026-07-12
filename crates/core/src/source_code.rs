use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadStatus {
  Ok,
  OpenFail,
  Oom,
  Oversized,
  Empty,
}

/// One extracted definition.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Definition {
  pub name: String,
  pub qualified_name: String,
  pub label: String,
  pub file_path: Option<String>,
  pub start_line: u32,
  pub end_line: u32,
  pub signature: Option<String>,
  pub return_type: Option<String>,
  pub parent_class: Option<String>,
  pub decorators: Vec<String>,
  pub base_classes: Vec<String>,
  pub param_names: Vec<String>,
  pub param_types: Vec<String>,
  pub return_types: Vec<String>,
  pub complexity: i32,
  pub lines: i32,
  pub is_exported: bool,
  pub is_test: bool,
  pub is_entry_point: bool,
}

/// Call site from extraction.
#[derive(Debug, Clone, Default)]
pub struct CallSite {
  pub callee_name: String,
  pub enclosing_func_qn: Option<String>,
  pub is_method: bool,
}

/// Resolved call from LSP / registry.
#[derive(Debug, Clone, Default)]
pub struct ResolvedCall {
  pub caller_qn: String,
  pub callee_name: String,
  pub qualified_name: String,
  pub strategy: String,
  pub confidence: f64,
  pub reason: Option<String>,
}

/// Import extracted from source.
#[derive(Debug, Clone, Default)]
pub struct Import {
  pub module_path: String,
  pub local_name: Option<String>,
  pub namespace: Option<String>,
}

/// Type usage edge candidate.
#[derive(Debug, Clone, Default)]
pub struct Usage {
  pub type_name: String,
  pub enclosing_func_qn: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ThrowSite {
  pub exception_type: String,
  pub enclosing_func_qn: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ReadWriteAccess {
  pub var_name: String,
  pub enclosing_func_qn: Option<String>,
  pub is_write: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChannelDirection {
  #[default]
  Emit,
  Listen,
}

#[derive(Debug, Clone, Default)]
pub struct Channel {
  pub channel_name: String,
  pub transport: Option<String>,
  pub direction: ChannelDirection,
  pub enclosing_func_qn: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EnvAccess {
  pub env_key: String,
  pub enclosing_func_qn: Option<String>,
}

/// Rust `impl Trait for Type` (raw names, matching CBM `CBMImplTrait`).
#[derive(Debug, Clone, Default)]
pub struct ImplTrait {
  pub trait_name: String,
  pub struct_name: String,
}

/// Per-file extraction result.
#[derive(Debug, Clone, Default)]
pub struct SourceCode {
  pub has_error: bool,
  pub error_msg: Option<String>,
  pub module_qn: Option<String>,
  pub rel_path: Option<String>,
  pub definitions: Vec<Definition>,
  pub calls: Vec<CallSite>,
  pub resolved_calls: Vec<ResolvedCall>,
  pub imports: Vec<Import>,
  pub usages: Vec<Usage>,
  pub throws: Vec<ThrowSite>,
  pub rw: Vec<ReadWriteAccess>,
  pub channels: Vec<Channel>,
  pub env_accesses: Vec<EnvAccess>,
  pub impl_traits: Vec<ImplTrait>,
}
