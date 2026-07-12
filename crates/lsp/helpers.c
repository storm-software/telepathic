#include "helpers.h"

#include <string.h>

#define GROW_ARRAY(arr, arena)                                                                   \
  do {                                                                                           \
    if ((arr)->count >= (arr)->cap) {                                                            \
      int new_cap = (arr)->cap == 0 ? CBM_SZ_32 : (arr)->cap * PAIR_LEN;                         \
      void *new_items = lsp_arena_alloc((arena), (size_t)new_cap * sizeof(*(arr)->items));       \
      if (!new_items)                                                                            \
        return;                                                                                  \
      if ((arr)->items && (arr)->count > 0) {                                                    \
        memcpy(new_items, (arr)->items, (size_t)(arr)->count * sizeof(*(arr)->items));           \
      }                                                                                          \
      (arr)->items = new_items;                                                                  \
      (arr)->cap = new_cap;                                                                      \
    }                                                                                            \
  } while (0)

void *lsp_memmem(const void *haystack, size_t haystack_len, const void *needle,
                 size_t needle_len) {
  if (needle_len == 0) {
    return (void *)haystack;
  }
  if (needle_len > haystack_len) {
    return NULL;
  }
  const char *h = (const char *)haystack;
  size_t last = haystack_len - needle_len;
  for (size_t i = 0; i <= last; i++) {
    if (memcmp(h + i, needle, needle_len) == 0) {
      return (void *)(h + i);
    }
  }
  return NULL;
}

char *lsp_node_text(CBMArena *a, TSNode node, const char *source) {
  uint32_t start = ts_node_start_byte(node);
  uint32_t end = ts_node_end_byte(node);
  if (end <= start) {
    return lsp_arena_strdup(a, "");
  }
  return lsp_arena_strndup(a, source + start, end - start);
}

bool lsp_label_is_type_like(const char *label) {
  if (!label) {
    return false;
  }
  return strcmp(label, "Class") == 0 || strcmp(label, "Struct") == 0 ||
         strcmp(label, "Interface") == 0 || strcmp(label, "Enum") == 0 ||
         strcmp(label, "Type") == 0 || strcmp(label, "Trait") == 0;
}

void lsp_defs_push(CBMDefArray *arr, CBMArena *a, CBMDefinition def) {
  GROW_ARRAY(arr, a);
  arr->items[arr->count++] = def;
}

void lsp_calls_push(CBMCallArray *arr, CBMArena *a, CBMCall call) {
  GROW_ARRAY(arr, a);
  arr->items[arr->count++] = call;
}

void lsp_imports_push(CBMImportArray *arr, CBMArena *a, CBMImport imp) {
  GROW_ARRAY(arr, a);
  arr->items[arr->count++] = imp;
}

void lsp_resolvedcall_push(CBMResolvedCallArray *arr, CBMArena *a, CBMResolvedCall rc) {
  GROW_ARRAY(arr, a);
  arr->items[arr->count++] = rc;
}

void lsp_impltrait_push(CBMImplTraitArray *arr, CBMArena *a, CBMImplTrait it) {
  GROW_ARRAY(arr, a);
  arr->items[arr->count++] = it;
}
