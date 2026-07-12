#ifndef CBM_H
#define CBM_H

#include "arena.h"
#include "tree_sitter/api.h"

#include <stdbool.h>
#include <stdint.h>

/* Language enum — values for LSP backends must match CBM naming. */
typedef enum {
  CBM_LANG_GO = 0,
  CBM_LANG_PYTHON,
  CBM_LANG_JAVASCRIPT,
  CBM_LANG_TYPESCRIPT,
  CBM_LANG_TSX,
  CBM_LANG_RUST,
  CBM_LANG_JAVA,
  CBM_LANG_CPP,
  CBM_LANG_CSHARP,
  CBM_LANG_PHP,
  CBM_LANG_LUA,
  CBM_LANG_SCALA,
  CBM_LANG_KOTLIN,
  CBM_LANG_RUBY,
  CBM_LANG_C,
  CBM_LANG_BASH,
  CBM_LANG_ZIG,
  CBM_LANG_ELIXIR,
  CBM_LANG_HASKELL,
  CBM_LANG_OCAML,
  CBM_LANG_OBJC,
  CBM_LANG_SWIFT,
  CBM_LANG_DART,
  CBM_LANG_PERL,
  CBM_LANG_GROOVY,
  CBM_LANG_ERLANG,
  CBM_LANG_R,
  CBM_LANG_HTML,
  CBM_LANG_CSS,
  CBM_LANG_SCSS,
  CBM_LANG_YAML,
  CBM_LANG_TOML,
  CBM_LANG_HCL,
  CBM_LANG_SQL,
  CBM_LANG_DOCKERFILE,
  CBM_LANG_CLOJURE,
  CBM_LANG_FSHARP,
  CBM_LANG_JULIA,
  CBM_LANG_VIMSCRIPT,
  CBM_LANG_NIX,
  CBM_LANG_COMMONLISP,
  CBM_LANG_ELM,
  CBM_LANG_FORTRAN,
  CBM_LANG_CUDA,
  CBM_LANG_COUNT
} CBMLanguage;

#define CBM_LSP_CONFIDENCE_FLOOR 0.6f
#define CBM_SZ_32 32
#define PAIR_LEN 2

typedef struct {
  const char *name;
  const char *qualified_name;
  const char *label;
  const char *file_path;
  uint32_t start_line;
  uint32_t end_line;
  const char *signature;
  const char *return_type;
  const char *receiver;
  const char *docstring;
  const char *parent_class;
  const char **decorators;
  const char **base_classes;
  const char **param_names;
  const char **param_types;
  const char **return_types;
  const char *route_path;
  const char *route_method;
  int complexity;
  int cognitive;
  int loop_count;
  int loop_depth;
  bool is_recursive;
  int param_count;
  int max_access_depth;
  int linear_scan_in_loop;
  int alloc_in_loop;
  bool recursion_in_loop;
  bool unguarded_recursion;
  int lines;
  uint32_t *fingerprint;
  int fingerprint_k;
  bool is_exported;
  bool is_abstract;
  bool is_test;
  bool is_entry_point;
  const char *structural_profile;
  const char *body_tokens;
} CBMDefinition;

typedef struct {
  const char *expr;
  const char *value;
  const char *keyword;
  int index;
} CBMCallArg;

#define CBM_MAX_CALL_ARGS 8

typedef struct {
  const char *callee_name;
  const char *enclosing_func_qn;
  const char *first_string_arg;
  const char *second_arg_name;
  CBMCallArg args[CBM_MAX_CALL_ARGS];
  int arg_count;
  int loop_depth;
  int branch_depth;
  int start_line;
  bool is_method;
} CBMCall;

typedef struct {
  const char *local_name;
  const char *module_path;
} CBMImport;

typedef struct {
  const char *ref_name;
  const char *enclosing_func_qn;
} CBMUsage;

typedef struct {
  const char *exception_name;
  const char *enclosing_func_qn;
} CBMThrow;

typedef struct {
  const char *var_name;
  const char *enclosing_func_qn;
  bool is_write;
} CBMReadWrite;

typedef struct {
  const char *type_name;
  const char *enclosing_func_qn;
} CBMTypeRef;

typedef struct {
  const char *env_key;
  const char *enclosing_func_qn;
} CBMEnvAccess;

typedef struct {
  const char *var_name;
  const char *type_name;
  const char *enclosing_func_qn;
} CBMTypeAssign;

typedef enum {
  CBM_STRREF_URL = 0,
  CBM_STRREF_CONFIG = 1,
} CBMStringRefKind;

typedef struct {
  const char *value;
  const char *enclosing_func_qn;
  const char *key_path;
  CBMStringRefKind kind;
} CBMStringRef;

typedef struct {
  const char *source_name;
  const char *target_url;
  const char *broker;
} CBMInfraBinding;

typedef enum {
  CBM_CHANNEL_EMIT = 0,
  CBM_CHANNEL_LISTEN = 1,
} CBMChannelDirection;

typedef struct {
  const char *channel_name;
  const char *transport;
  const char *enclosing_func_qn;
  CBMChannelDirection direction;
} CBMChannel;

typedef struct {
  const char *trait_name;
  const char *struct_name;
} CBMImplTrait;

typedef struct {
  const char *caller_qn;
  const char *callee_qn;
  const char *strategy;
  float confidence;
  const char *reason;
} CBMResolvedCall;

typedef struct {
  CBMResolvedCall *items;
  int count;
  int cap;
} CBMResolvedCallArray;

typedef struct {
  CBMDefinition *items;
  int count;
  int cap;
} CBMDefArray;

typedef struct {
  CBMCall *items;
  int count;
  int cap;
} CBMCallArray;

typedef struct {
  CBMImport *items;
  int count;
  int cap;
} CBMImportArray;

typedef struct {
  CBMUsage *items;
  int count;
  int cap;
} CBMUsageArray;

typedef struct {
  CBMThrow *items;
  int count;
  int cap;
} CBMThrowArray;

typedef struct {
  CBMReadWrite *items;
  int count;
  int cap;
} CBMRWArray;

typedef struct {
  CBMTypeRef *items;
  int count;
  int cap;
} CBMTypeRefArray;

typedef struct {
  CBMEnvAccess *items;
  int count;
  int cap;
} CBMEnvAccessArray;

typedef struct {
  CBMTypeAssign *items;
  int count;
  int cap;
} CBMTypeAssignArray;

typedef struct {
  CBMStringRef *items;
  int count;
  int cap;
} CBMStringRefArray;

typedef struct {
  CBMInfraBinding *items;
  int count;
  int cap;
} CBMInfraBindingArray;

typedef struct {
  CBMImplTrait *items;
  int count;
  int cap;
} CBMImplTraitArray;

typedef struct {
  CBMChannel *items;
  int count;
  int cap;
} CBMChannelArray;

typedef struct {
  CBMArena arena;
  CBMDefArray defs;
  CBMCallArray calls;
  CBMImportArray imports;
  CBMUsageArray usages;
  CBMThrowArray throws;
  CBMRWArray rw;
  CBMTypeRefArray type_refs;
  CBMEnvAccessArray env_accesses;
  CBMTypeAssignArray type_assigns;
  CBMImplTraitArray impl_traits;
  CBMResolvedCallArray resolved_calls;
  CBMStringRefArray string_refs;
  CBMInfraBindingArray infra_bindings;
  CBMChannelArray channels;
  const char *module_qn;
  const char *namespace_name;
  const char **exports;
  const char **constants;
  const char **global_vars;
  const char **macros;
  bool has_error;
  const char *error_msg;
  bool parse_incomplete;
  const char *error_ranges;
  int error_region_count;
  bool is_test_file;
  int imports_count;
  TSTree *cached_tree;
  CBMLanguage cached_lang;
  const char *source;
  int source_len;
} CBMFileResult;

void lsp_defs_push(CBMDefArray *arr, CBMArena *a, CBMDefinition def);
void lsp_calls_push(CBMCallArray *arr, CBMArena *a, CBMCall call);
void lsp_imports_push(CBMImportArray *arr, CBMArena *a, CBMImport imp);
void lsp_resolvedcall_push(CBMResolvedCallArray *arr, CBMArena *a, CBMResolvedCall rc);
void lsp_impltrait_push(CBMImplTraitArray *arr, CBMArena *a, CBMImplTrait it);

bool lsp_label_is_type_like(const char *label);

#endif /* CBM_H */
