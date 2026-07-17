use std::{
  env,
  path::{Path, PathBuf},
};

fn main() {
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let workspace = find_workspace(&manifest_dir);
  let vendored = manifest_dir.join("vendored");
  let ts_include = find_tree_sitter_include();

  println!("cargo:rerun-if-changed={}", vendored.display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("arena.c").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("helpers.c").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("cbm.h").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("arena.h").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("dispatch.c").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("helpers.h").display());
  println!("cargo:rerun-if-changed={}", manifest_dir.join("wrapper.h").display());

  compile_grammars(&workspace);
  compile_lsp_runtime(&manifest_dir, &vendored, &ts_include);
  generate_bindings(&manifest_dir, &ts_include);
}

fn generate_bindings(manifest_dir: &Path, ts_include: &Path) {
  let wrapper = manifest_dir.join("wrapper.h");
  // bindgen only parses headers for Rust FFI. When TARGET is wasm32, libclang
  // inherits that target and cannot find stdlib.h (no wasi sysroot for bindgen).
  // Force host triple so host libc headers resolve.
  let mut builder = bindgen::Builder::default()
    .header(wrapper.to_string_lossy())
    .clang_arg(format!("-I{}", manifest_dir.display()))
    .clang_arg(format!("-I{}", ts_include.display()));

  let target = env::var("TARGET").unwrap_or_default();
  if target.contains("wasm") {
    let host = env::var("HOST").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".into());
    builder = builder.clang_arg(format!("--target={host}"));
  }

  let bindings = builder
    .allowlist_function("lsp_.*")
    .allowlist_type("CBM.*")
    .allowlist_type("TSNode")
    .allowlist_var("CBM_.*")
    .derive_default(true)
    .derive_debug(true)
    .generate()
    .expect("bindgen failed for telepathic-lsp wrapper.h");

  let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
  bindings.write_to_file(&out).expect("write bindings.rs");
}

fn find_workspace(manifest_dir: &Path) -> PathBuf {
  let mut workspace = manifest_dir.to_path_buf();
  while !workspace.join(".git").exists()
    && !workspace.join(".github").exists()
    && workspace.parent().is_some()
  {
    workspace = workspace.parent().unwrap().to_path_buf();
  }
  workspace
}

fn find_tree_sitter_include() -> PathBuf {
  if let Ok(p) = env::var("DEP_TREE_SITTER_INCLUDE") {
    return PathBuf::from(p);
  }
  let cargo_home = env::var("CARGO_HOME").unwrap_or_else(|_| dirs_fallback());
  let registry = PathBuf::from(cargo_home).join("registry/src");
  if let Ok(entries) = std::fs::read_dir(&registry) {
    for entry in entries.flatten() {
      let candidate = entry.path().join("tree-sitter-0.26.10/include");
      if candidate.join("tree_sitter/api.h").is_file() {
        return candidate;
      }
    }
  }
  panic!(
    "tree-sitter 0.26.10 headers not found; set DEP_TREE_SITTER_INCLUDE or cargo fetch tree-sitter"
  );
}

fn dirs_fallback() -> String {
  env::var("HOME")
    .map(|h| format!("{h}/.cargo"))
    .unwrap_or_else(|_| "/home/development/.cargo".into())
}

fn target_is_msvc() -> bool {
  // Cross-compile: build script runs on host, so cfg(target_env = "msvc") is wrong.
  // CARGO_CFG_TARGET_ENV reflects the crate being built.
  env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc")
}

fn compile_grammars(workspace: &Path) {
  let grammars_root = workspace.join("crates/tree-sitter/vendored");
  let keys = [
    "c",
    "cpp",
    "c_sharp",
    "go",
    "java",
    "javascript",
    "kotlin",
    "perl",
    "php",
    "python",
    "rust",
    "tsx",
    "typescript",
  ];
  let msvc = target_is_msvc();
  for key in keys {
    let grammar_dir = grammars_root.join(key);
    let parser_path = grammar_dir.join("parser.c");
    if !parser_path.is_file() {
      panic!("missing grammar parser: {}", parser_path.display());
    }
    let mut build = cc::Build::new();
    build
      .std("c11")
      .include(&grammar_dir)
      .file(&parser_path)
      .flag_if_supported("-Wno-unused-parameter")
      .flag_if_supported("-Wno-unused-but-set-variable")
      .flag_if_supported("-Wno-unused-variable")
      .flag_if_supported("-Wno-unused-function")
      .flag_if_supported("-Wno-trigraphs");
    if msvc {
      build.flag("-utf-8");
    }

    let scanner = grammar_dir.join("scanner.c");
    if scanner.is_file() {
      build.file(&scanner);
      println!("cargo:rerun-if-changed={}", scanner.display());
    }
    println!("cargo:rerun-if-changed={}", parser_path.display());
    build.compile(&format!("telepathic-lsp-grammar-{key}"));
  }
}

fn compile_lsp_runtime(manifest_dir: &Path, vendored: &Path, ts_include: &Path) {
  let helpers_h = manifest_dir.join("helpers.h");
  let helpers_h = helpers_h.to_str().expect("helpers.h path must be UTF-8");
  let mut build = cc::Build::new();
  build
    .std("c11")
    .include(manifest_dir)
    .include(vendored)
    .include(ts_include)
    .define("_GNU_SOURCE", None)
    .flag_if_supported("-Wno-unused-parameter")
    .flag_if_supported("-Wno-unused-but-set-variable")
    .flag_if_supported("-Wno-unused-variable")
    .flag_if_supported("-Wno-unused-function")
    .flag_if_supported("-Wno-sign-compare")
    .flag_if_supported("-Wno-pedantic")
    .flag_if_supported("-Wno-implicit-fallthrough")
    .flag_if_supported("-Wno-format-truncation")
    .flag_if_supported("-Wno-error=implicit-function-declaration");

  // GNU `-include` is ignored by clang-cl; next arg becomes a bogus extra source
  // → "cannot specify -Fo when compiling multiple source files". Use /FI on MSVC.
  if target_is_msvc() {
    build.flag(format!("/FI{helpers_h}"));
    build.flag("-utf-8");
  } else {
    build.flag("-include").flag(helpers_h);
  }

  let shim_files = ["arena.c", "helpers.c", "dispatch.c"];
  for name in shim_files {
    let path = manifest_dir.join(name);
    build.file(&path);
    println!("cargo:rerun-if-changed={}", path.display());
  }

  let runtime_files = [
    "type_rep.c",
    "type_registry.c",
    "scope.c",
    "go_lsp.c",
    "c_lsp.c",
    "php_lsp.c",
    "perl_lsp.c",
    "py_lsp.c",
    "ts_lsp.c",
    "cs_lsp.c",
    "java_lsp.c",
    "kotlin_lsp.c",
    "rust_lsp.c",
    "rust_cargo.c",
    "rust_proc_macros.c",
    "generated/c_stdlib_data.c",
    "generated/cpp_stdlib_data.c",
    "generated/cs_stdlib_data.c",
    "generated/go_stdlib_data.c",
    "generated/java_stdlib_data.c",
    "generated/kotlin_stdlib_data.c",
    "generated/perl_stdlib_data.c",
    "generated/php_stdlib_data.c",
    "generated/python_stdlib_data.c",
    "generated/rust_stdlib_data.c",
    "generated/rust_crates_seed.c",
  ];

  for name in runtime_files {
    let path = vendored.join(name);
    if !path.is_file() {
      panic!("missing vendored source: {}", path.display());
    }
    build.file(&path);
    println!("cargo:rerun-if-changed={}", path.display());
  }

  build.compile("telepathic-lsp-cbm");
}
