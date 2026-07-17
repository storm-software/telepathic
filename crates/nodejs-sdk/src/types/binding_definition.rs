use napi_derive::napi;
use telepathic_core::Definition;

#[derive(Debug, Clone, PartialEq)]
#[napi(object)]
pub struct BindingDefinition {
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

impl From<BindingDefinition> for Definition {
  fn from(value: BindingDefinition) -> Self {
    Self {
      name: value.name,
      qualified_name: value.qualified_name,
      label: value.label,
      file_path: value.file_path,
      start_line: value.start_line,
      end_line: value.end_line,
      signature: value.signature,
      return_type: value.return_type,
      parent_class: value.parent_class,
      decorators: value.decorators,
      base_classes: value.base_classes,
      param_names: value.param_names,
      param_types: value.param_types,
      return_types: value.return_types,
      complexity: value.complexity,
      lines: value.lines,
      is_exported: value.is_exported,
      is_test: value.is_test,
      is_entry_point: value.is_entry_point,
    }
  }
}

impl From<Definition> for BindingDefinition {
  fn from(value: Definition) -> Self {
    Self {
      name: value.name,
      qualified_name: value.qualified_name,
      label: value.label,
      file_path: value.file_path,
      start_line: value.start_line,
      end_line: value.end_line,
      signature: value.signature,
      return_type: value.return_type,
      parent_class: value.parent_class,
      decorators: value.decorators,
      base_classes: value.base_classes,
      param_names: value.param_names,
      param_types: value.param_types,
      return_types: value.return_types,
      complexity: value.complexity,
      lines: value.lines,
      is_exported: value.is_exported,
      is_test: value.is_test,
      is_entry_point: value.is_entry_point,
    }
  }
}
