#include "wrapper.h"

#include "vendored/c_lsp.h"
#include "vendored/cs_lsp.h"
#include "vendored/go_lsp.h"
#include "vendored/java_lsp.h"
#include "vendored/kotlin_lsp.h"
#include "vendored/perl_lsp.h"
#include "vendored/php_lsp.h"
#include "vendored/py_lsp.h"
#include "vendored/rust_lsp.h"
#include "vendored/ts_lsp.h"

void lsp_run_go(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                TSNode root) {
  cbm_run_go_lsp(arena, result, source, source_len, root);
}

void lsp_run_c(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
               TSNode root, bool cpp_mode) {
  cbm_run_c_lsp(arena, result, source, source_len, root, cpp_mode);
}

void lsp_run_php(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                 TSNode root) {
  cbm_run_php_lsp(arena, result, source, source_len, root);
}

void lsp_run_perl(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                  TSNode root) {
  cbm_run_perl_lsp(arena, result, source, source_len, root);
}

void lsp_run_py(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                TSNode root) {
  cbm_run_py_lsp(arena, result, source, source_len, root);
}

void lsp_run_ts(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                TSNode root, bool js_mode, bool jsx_mode, bool dts_mode) {
  cbm_run_ts_lsp(arena, result, source, source_len, root, js_mode, jsx_mode, dts_mode);
}

void lsp_run_cs(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                TSNode root) {
  cbm_run_cs_lsp(arena, result, source, source_len, root);
}

void lsp_run_java(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                  TSNode root) {
  cbm_run_java_lsp(arena, result, source, source_len, root);
}

void lsp_run_kotlin(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                    TSNode root) {
  cbm_run_kotlin_lsp(arena, result, source, source_len, root);
}

void lsp_run_rust(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                  TSNode root) {
  cbm_run_rust_lsp(arena, result, source, source_len, root);
}
