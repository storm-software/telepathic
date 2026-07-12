#ifndef CBM_ARENA_H
#define CBM_ARENA_H

#include <stddef.h>

/* Bump allocator: all freed via lsp_arena_destroy. */
#define CBM_ARENA_MAX_BLOCKS 256
#define CBM_ARENA_DEFAULT_BLOCK_SIZE (64 * 1024)

typedef struct {
  char *blocks[CBM_ARENA_MAX_BLOCKS];
  size_t block_sizes[CBM_ARENA_MAX_BLOCKS];
  int nblocks;
  size_t block_size;
  size_t used;
  size_t total_alloc;
} CBMArena;

void lsp_arena_init(CBMArena *a);
void *lsp_arena_alloc(CBMArena *a, size_t n);
char *lsp_arena_strdup(CBMArena *a, const char *s);
char *lsp_arena_strndup(CBMArena *a, const char *s, size_t len);
char *lsp_arena_sprintf(CBMArena *a, const char *fmt, ...)
#if defined(__GNUC__) || defined(__clang__)
  __attribute__((format(printf, 2, 3)))
#endif
  ;
void lsp_arena_destroy(CBMArena *a);

/* Vendored LSP sources use the legacy cbm_arena_* names. */
#define cbm_arena_init lsp_arena_init
#define cbm_arena_alloc lsp_arena_alloc
#define cbm_arena_strdup lsp_arena_strdup
#define cbm_arena_strndup lsp_arena_strndup
#define cbm_arena_sprintf lsp_arena_sprintf
#define cbm_arena_destroy lsp_arena_destroy

#endif /* CBM_ARENA_H */
