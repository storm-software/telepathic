/// Languages with a vendored per-file LSP backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LspLanguage {
  Go,
  Python,
  JavaScript,
  TypeScript,
  Tsx,
  Rust,
  Java,
  Cpp,
  C,
  Cuda,
  CSharp,
  Php,
  Perl,
  Kotlin,
}

impl LspLanguage {
  pub(crate) const fn lsp_lang(self) -> u32 {
    match self {
      Self::Go => 0,
      Self::Python => 1,
      Self::JavaScript => 2,
      Self::TypeScript => 3,
      Self::Tsx => 4,
      Self::Rust => 5,
      Self::Java => 6,
      Self::Cpp => 7,
      Self::CSharp => 8,
      Self::Php => 9,
      Self::Kotlin => 12,
      Self::C => 14,
      Self::Perl => 23,
      Self::Cuda => 43,
    }
  }

  pub(crate) fn ts_modes(self, rel_path: Option<&str>) -> (bool, bool, bool) {
    let mut js_mode = matches!(self, Self::JavaScript);
    let mut jsx_mode = matches!(self, Self::Tsx);
    let mut dts_mode = false;

    if let Some(path) = rel_path {
      if matches!(self, Self::JavaScript) && path.ends_with(".jsx") {
        jsx_mode = true;
      }
      if matches!(self, Self::TypeScript) && path.ends_with(".d.ts") {
        dts_mode = true;
      }
      if path.ends_with(".jsx") {
        js_mode = true;
        jsx_mode = true;
      }
    }

    (js_mode, jsx_mode, dts_mode)
  }
}
