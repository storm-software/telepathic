# telepathic-semantic


## External integrations

| Need                     | crates.io crate?                                                                            | What we use                                                                                                                                                                                    |
| ------------------------ | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AST MinHash              | **Yes** — [`normalize-code-similarity`](https://crates.io/crates/normalize-code-similarity) | Feature `ast-minhash` (default): `compute_ast_minhash`, `ast_minhash_jaccard`                                                                                                                  |
| nomic `code_vectors.bin` | **No**                                                                                      | Feature `nomic-pretrained`: load `code_tokens.txt` + `code_vectors.bin` from [codebase-memory-mcp `vendored/nomic/`](https://github.com/DeusData/codebase-memory-mcp/tree/main/vendored/nomic) |

`funcvec` / `fastembed` run full nomic models at runtime — not the static 40K×768 int8 lookup table CBM uses.

### Nomic pretrained setup

```bash
export TELEPATHIC_NOMIC_DATA_DIR=/path/to/vendored/nomic
# directory must contain code_tokens.txt and code_vectors.bin
```

```rust
use telepathic_semantic::{NomicPretrained, default_pretrained};

let pretrained = NomicPretrained::from_dir("vendored/nomic")?;
corpus.finalize(&pretrained);
```

Or set `TELEPATHIC_NOMIC_DATA_DIR` and use `default_pretrained()`.

The static int8 table (`code_vectors.bin`, 40K×768) is a distilled token lookup for Random Indexing seeds — **not** the full neural CodeRankEmbed / nomic-embed-code forward pass. Full neural inference lives in `telepathic-embedding`.

### AST MinHash

```rust
use telepathic_semantic::{compute_ast_minhash, ast_minhash_jaccard};

let sig = compute_ast_minhash(&func_body, source.as_bytes())?;
let similarity = ast_minhash_jaccard(&sig_a, &sig_b);
```

Not wire-compatible with CBM `minhash.c` (K=64 u32 xxHash trigrams). Signatures are consistent within this Rust path.
