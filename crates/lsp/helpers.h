#ifndef CBM_HELPERS_H
#define CBM_HELPERS_H

#include "cbm.h"

#include <stddef.h>

void *lsp_memmem(const void *haystack, size_t haystack_len, const void *needle,
                 size_t needle_len);

char *lsp_node_text(CBMArena *a, TSNode node, const char *source);

#endif /* CBM_HELPERS_H */
