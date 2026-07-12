//! Public per-file LSP resolve entry point.

use tree_sitter::Tree;

use telepathic_core::SourceCode;

use crate::{
  convert::{OwnedCStrings, build_file_result, destroy_file_result, sync_back},
  error::LspError,
  ffi::{self, TSNode},
  language::LspLanguage,
};

/// Run per-file CBM LSP resolution and sync results into `source`.
///
/// Requires `source.module_qn` (and preferably `source.rel_path` for TS/JS modes).
/// Caller must keep `tree` alive for the duration of the call.
pub fn resolve(
  lang: LspLanguage,
  source_bytes: &[u8],
  tree: &Tree,
  source: &mut SourceCode,
) -> Result<(), LspError> {
  let mut keep = OwnedCStrings::new();
  let result = unsafe { build_file_result(source, source_bytes, &mut keep)? };
  let root = tree.root_node().into_raw();
  debug_assert_eq!(std::mem::size_of_val(&root), std::mem::size_of::<TSNode>());
  let ts_node: TSNode = unsafe { std::mem::transmute(root) };

  let arena = unsafe { std::ptr::addr_of_mut!((*result).arena) };
  let len = i32::try_from(source_bytes.len()).unwrap_or(i32::MAX);
  let src = source_bytes.as_ptr().cast::<std::os::raw::c_char>();

  unsafe {
    dispatch(lang, arena, result, src, len, ts_node, source.rel_path.as_deref());
    sync_back(result, source);
    destroy_file_result(result);
  }

  Ok(())
}

unsafe fn dispatch(
  lang: LspLanguage,
  arena: *mut ffi::CBMArena,
  result: *mut ffi::CBMFileResult,
  src: *const std::os::raw::c_char,
  len: i32,
  root: TSNode,
  rel_path: Option<&str>,
) {
  match lang {
    LspLanguage::Go => unsafe { ffi::lsp_run_go(arena, result, src, len, root) },
    LspLanguage::C => unsafe { ffi::lsp_run_c(arena, result, src, len, root, false) },
    LspLanguage::Cpp | LspLanguage::Cuda => unsafe {
      ffi::lsp_run_c(arena, result, src, len, root, true)
    },
    LspLanguage::Php => unsafe { ffi::lsp_run_php(arena, result, src, len, root) },
    LspLanguage::Perl => unsafe { ffi::lsp_run_perl(arena, result, src, len, root) },
    LspLanguage::Python => unsafe { ffi::lsp_run_py(arena, result, src, len, root) },
    LspLanguage::JavaScript | LspLanguage::TypeScript | LspLanguage::Tsx => {
      let (js_mode, jsx_mode, dts_mode) = lang.ts_modes(rel_path);
      unsafe { ffi::lsp_run_ts(arena, result, src, len, root, js_mode, jsx_mode, dts_mode) }
    }
    LspLanguage::CSharp => unsafe { ffi::lsp_run_cs(arena, result, src, len, root) },
    LspLanguage::Java => unsafe { ffi::lsp_run_java(arena, result, src, len, root) },
    LspLanguage::Kotlin => unsafe { ffi::lsp_run_kotlin(arena, result, src, len, root) },
    LspLanguage::Rust => unsafe { lsp_run_rust(arena, result, src, len, root) },
  }
}
