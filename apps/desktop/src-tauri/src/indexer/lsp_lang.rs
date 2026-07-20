//! Map tree-sitter [`Language`] to LSP backends.

use telepathic_lsp::LspLanguage;
use telepathic_tree_sitter::Language;

pub(super) fn to_lsp_language(lang: Language) -> Option<LspLanguage> {
  match lang {
    Language::Go => Some(LspLanguage::Go),
    Language::Python => Some(LspLanguage::Python),
    Language::JavaScript => Some(LspLanguage::JavaScript),
    Language::TypeScript => Some(LspLanguage::TypeScript),
    Language::Tsx => Some(LspLanguage::Tsx),
    Language::Rust => Some(LspLanguage::Rust),
    Language::Java => Some(LspLanguage::Java),
    Language::Cpp => Some(LspLanguage::Cpp),
    Language::C => Some(LspLanguage::C),
    Language::Cuda => Some(LspLanguage::Cuda),
    Language::CSharp => Some(LspLanguage::CSharp),
    Language::Php => Some(LspLanguage::Php),
    Language::Perl => Some(LspLanguage::Perl),
    Language::Kotlin => Some(LspLanguage::Kotlin),
    _ => None,
  }
}
