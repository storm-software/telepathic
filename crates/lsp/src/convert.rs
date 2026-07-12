//! Map Rust [`SourceCode`] ↔ C `CBMFileResult`.

use std::{
  alloc::{Layout, alloc_zeroed, dealloc},
  ffi::{CStr, CString},
  os::raw::c_char,
  ptr, slice,
};

use telepathic_core::{CallSite, Definition, ImplTrait, Import, ResolvedCall, SourceCode};

use crate::{
  error::LspError,
  ffi::{self, CBMCall, CBMDefinition, CBMFileResult, CBMImplTrait, CBMImport},
};

pub(crate) struct OwnedCStrings {
  strings: Vec<CString>,
  tables: Vec<(*mut *const c_char, usize)>,
}

impl OwnedCStrings {
  pub(crate) fn new() -> Self {
    Self { strings: Vec::new(), tables: Vec::new() }
  }

  fn push(&mut self, s: &str) -> Result<*const c_char, LspError> {
    let c = CString::new(s).map_err(|_| LspError::InteriorNul)?;
    let p = c.as_ptr();
    self.strings.push(c);
    Ok(p)
  }

  fn push_opt(&mut self, s: Option<&str>) -> Result<*const c_char, LspError> {
    match s {
      Some(v) => self.push(v),
      None => Ok(ptr::null()),
    }
  }

  fn push_list(&mut self, items: &[String]) -> Result<*mut *const c_char, LspError> {
    if items.is_empty() {
      return Ok(ptr::null_mut());
    }
    let mut ptrs = Vec::with_capacity(items.len() + 1);
    for item in items {
      ptrs.push(self.push(item)?);
    }
    ptrs.push(ptr::null());
    let len = ptrs.len();
    let boxed = ptrs.into_boxed_slice();
    let raw = Box::into_raw(boxed) as *mut *const c_char;
    self.tables.push((raw, len));
    Ok(raw)
  }
}

impl Drop for OwnedCStrings {
  fn drop(&mut self) {
    for &(raw, len) in &self.tables {
      if raw.is_null() {
        continue;
      }
      unsafe {
        drop(Box::from_raw(slice::from_raw_parts_mut(raw, len)));
      }
    }
  }
}

/// Build a zeroed `CBMFileResult`, fill from `source`, init arena.
pub(crate) unsafe fn build_file_result(
  source: &SourceCode,
  source_bytes: &[u8],
  keep: &mut OwnedCStrings,
) -> Result<*mut CBMFileResult, LspError> {
  let result = unsafe { alloc_result() };
  if result.is_null() {
    return Err(LspError::ArenaOom);
  }

  unsafe {
    ffi::lsp_arena_init(ptr::addr_of_mut!((*result).arena));
  }

  let module_qn = source.module_qn.as_deref().ok_or(LspError::MissingModuleQn)?;
  unsafe {
    (*result).module_qn = keep.push(module_qn)?;
    (*result).source = source_bytes.as_ptr().cast::<c_char>();
    (*result).source_len = i32::try_from(source_bytes.len()).unwrap_or(i32::MAX);
  }

  unsafe {
    fill_defs(result, &source.definitions, keep)?;
    fill_calls(result, &source.calls, keep)?;
    fill_imports(result, &source.imports, keep)?;
    fill_impl_traits(result, &source.impl_traits, keep)?;
  }

  Ok(result)
}

unsafe fn fill_defs(
  result: *mut CBMFileResult,
  defs: &[Definition],
  keep: &mut OwnedCStrings,
) -> Result<(), LspError> {
  for def in defs {
    let mut d: CBMDefinition = unsafe { std::mem::zeroed() };
    d.name = keep.push(&def.name)?;
    d.qualified_name = keep.push(&def.qualified_name)?;
    d.label = keep.push(&def.label)?;
    d.file_path = keep.push_opt(def.file_path.as_deref())?;
    d.start_line = def.start_line;
    d.end_line = def.end_line;
    d.signature = keep.push_opt(def.signature.as_deref())?;
    d.return_type = keep.push_opt(def.return_type.as_deref())?;
    d.parent_class = keep.push_opt(def.parent_class.as_deref())?;
    d.decorators = keep.push_list(&def.decorators)?;
    d.base_classes = keep.push_list(&def.base_classes)?;
    d.param_names = keep.push_list(&def.param_names)?;
    d.param_types = keep.push_list(&def.param_types)?;
    d.return_types = keep.push_list(&def.return_types)?;
    d.complexity = def.complexity;
    d.lines = def.lines;
    d.is_exported = def.is_exported;
    d.is_test = def.is_test;
    d.is_entry_point = def.is_entry_point;
    unsafe {
      ffi::lsp_defs_push(ptr::addr_of_mut!((*result).defs), ptr::addr_of_mut!((*result).arena), d);
    }
  }
  Ok(())
}

unsafe fn fill_calls(
  result: *mut CBMFileResult,
  calls: &[CallSite],
  keep: &mut OwnedCStrings,
) -> Result<(), LspError> {
  for call in calls {
    let mut c: CBMCall = unsafe { std::mem::zeroed() };
    c.callee_name = keep.push(&call.callee_name)?;
    c.enclosing_func_qn = keep.push_opt(call.enclosing_func_qn.as_deref())?;
    c.is_method = call.is_method;
    unsafe {
      ffi::lsp_calls_push(
        ptr::addr_of_mut!((*result).calls),
        ptr::addr_of_mut!((*result).arena),
        c,
      );
    }
  }
  Ok(())
}

unsafe fn fill_imports(
  result: *mut CBMFileResult,
  imports: &[Import],
  keep: &mut OwnedCStrings,
) -> Result<(), LspError> {
  for imp in imports {
    let mut i: CBMImport = unsafe { std::mem::zeroed() };
    i.module_path = keep.push(&imp.module_path)?;
    i.local_name = keep.push_opt(imp.local_name.as_deref())?;
    unsafe {
      ffi::lsp_imports_push(
        ptr::addr_of_mut!((*result).imports),
        ptr::addr_of_mut!((*result).arena),
        i,
      );
    }
  }
  Ok(())
}

unsafe fn fill_impl_traits(
  result: *mut CBMFileResult,
  items: &[ImplTrait],
  keep: &mut OwnedCStrings,
) -> Result<(), LspError> {
  for it in items {
    let mut row: CBMImplTrait = unsafe { std::mem::zeroed() };
    row.trait_name = keep.push(&it.trait_name)?;
    row.struct_name = keep.push(&it.struct_name)?;
    unsafe {
      ffi::lsp_impltrait_push(
        ptr::addr_of_mut!((*result).impl_traits),
        ptr::addr_of_mut!((*result).arena),
        row,
      );
    }
  }
  Ok(())
}

/// Copy C arrays back into `source` (replace defs/calls/resolved_calls).
pub(crate) unsafe fn sync_back(result: *mut CBMFileResult, source: &mut SourceCode) {
  unsafe {
    source.definitions = read_defs(result);
    source.calls = read_calls(result);
    source.resolved_calls = read_resolved(result);
  }
}

unsafe fn read_defs(result: *const CBMFileResult) -> Vec<Definition> {
  let arr = unsafe { &(*result).defs };
  if arr.items.is_null() || arr.count <= 0 {
    return Vec::new();
  }
  let slice = unsafe { slice::from_raw_parts(arr.items, arr.count as usize) };
  slice
    .iter()
    .map(|d| Definition {
      name: cstr(d.name),
      qualified_name: cstr(d.qualified_name),
      label: cstr(d.label),
      file_path: cstr_opt(d.file_path),
      start_line: d.start_line,
      end_line: d.end_line,
      signature: cstr_opt(d.signature),
      return_type: cstr_opt(d.return_type),
      parent_class: cstr_opt(d.parent_class),
      decorators: cstr_list(d.decorators),
      base_classes: cstr_list(d.base_classes),
      param_names: cstr_list(d.param_names),
      param_types: cstr_list(d.param_types),
      return_types: cstr_list(d.return_types),
      complexity: d.complexity,
      lines: d.lines,
      is_exported: d.is_exported,
      is_test: d.is_test,
      is_entry_point: d.is_entry_point,
    })
    .collect()
}

unsafe fn read_calls(result: *const CBMFileResult) -> Vec<CallSite> {
  let arr = unsafe { &(*result).calls };
  if arr.items.is_null() || arr.count <= 0 {
    return Vec::new();
  }
  let slice = unsafe { slice::from_raw_parts(arr.items, arr.count as usize) };
  slice
    .iter()
    .map(|c| CallSite {
      callee_name: cstr(c.callee_name),
      enclosing_func_qn: cstr_opt(c.enclosing_func_qn),
      is_method: c.is_method,
    })
    .collect()
}

unsafe fn read_resolved(result: *const CBMFileResult) -> Vec<ResolvedCall> {
  let arr = unsafe { &(*result).resolved_calls };
  if arr.items.is_null() || arr.count <= 0 {
    return Vec::new();
  }
  let slice = unsafe { slice::from_raw_parts(arr.items, arr.count as usize) };
  slice
    .iter()
    .map(|rc| {
      let qualified_name = cstr(rc.callee_qn);
      let callee_name =
        qualified_name.rsplit('.').next().unwrap_or(qualified_name.as_str()).to_string();
      ResolvedCall {
        caller_qn: cstr(rc.caller_qn),
        callee_name,
        qualified_name,
        strategy: cstr(rc.strategy),
        confidence: f64::from(rc.confidence),
        reason: cstr_opt(rc.reason),
      }
    })
    .collect()
}

fn cstr(p: *const c_char) -> String {
  if p.is_null() {
    return String::new();
  }
  unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

fn cstr_opt(p: *const c_char) -> Option<String> {
  if p.is_null() { None } else { Some(cstr(p)) }
}

fn cstr_list(p: *const *const c_char) -> Vec<String> {
  if p.is_null() {
    return Vec::new();
  }
  let mut out = Vec::new();
  let mut i = 0isize;
  loop {
    let item = unsafe { *p.offset(i) };
    if item.is_null() {
      break;
    }
    out.push(cstr(item));
    i += 1;
  }
  out
}

pub(crate) unsafe fn destroy_file_result(result: *mut CBMFileResult) {
  if result.is_null() {
    return;
  }
  unsafe {
    ffi::lsp_arena_destroy(ptr::addr_of_mut!((*result).arena));
    free_result(result);
  }
}

unsafe fn alloc_result() -> *mut CBMFileResult {
  let layout = Layout::new::<CBMFileResult>();
  let p = unsafe { alloc_zeroed(layout) }.cast::<CBMFileResult>();
  if p.is_null() { ptr::null_mut() } else { p }
}

unsafe fn free_result(p: *mut CBMFileResult) {
  let layout = Layout::new::<CBMFileResult>();
  unsafe { dealloc(p.cast::<u8>(), layout) };
}
