/* wrapper.h — bindgen entry for telepathic-lsp FFI surface. */
#include "cbm.h"
#include "helpers.h"

void lsp_run_go(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                    TSNode root);
void lsp_run_c(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                   TSNode root, bool cpp_mode);
void lsp_run_php(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                     TSNode root);
void lsp_run_perl(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                      TSNode root);
void lsp_run_py(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                    TSNode root);
void lsp_run_ts(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                    TSNode root, bool js_mode, bool jsx_mode, bool dts_mode);
void lsp_run_cs(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                    TSNode root);
void lsp_run_java(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                      TSNode root);
void lsp_run_kotlin(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                        TSNode root);
void lsp_run_rust(CBMArena *arena, CBMFileResult *result, const char *source, int source_len,
                      TSNode root);
