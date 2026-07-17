#ifndef CBM_HELPERS_H
#define CBM_HELPERS_H

#include "cbm.h"

#include <stddef.h>
#include <string.h>

/* MSVC has strtok_s with the same signature as POSIX strtok_r. */
#if defined(_MSC_VER) && !defined(strtok_r)
#define strtok_r strtok_s
#endif

void *lsp_memmem(const void *haystack, size_t haystack_len, const void *needle,
                 size_t needle_len);

char *lsp_node_text(CBMArena *a, TSNode node, const char *source);

static inline char *cbm_node_text(CBMArena *a, TSNode node, const char *source) {
  return lsp_node_text(a, node, source);
}

/* Vendored LSP sources use the legacy cbm_* helper names. */
#define cbm_memmem lsp_memmem
#define cbm_defs_push lsp_defs_push
#define cbm_calls_push lsp_calls_push
#define cbm_imports_push lsp_imports_push
#define cbm_resolvedcall_push lsp_resolvedcall_push
#define cbm_impltrait_push lsp_impltrait_push
#define cbm_label_is_type_like lsp_label_is_type_like

#endif /* CBM_HELPERS_H */
