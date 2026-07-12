//! CFML language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add the CFML language to a
//! tree-sitter [`Parser`][], and then use the parser to parse some code:
//!
//! ```
//! let code = "";
//! let mut parser = tree_sitter::Parser::new();
//! parser
//!     .set_language(&tree_sitter_cfml::LANGUAGE.into())
//!     .expect("Error loading CFML grammar");
//! let tree = parser.parse(code, None).unwrap();
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_cfml() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub(crate) const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_cfml) };

pub(crate) const BRACKETS_ZED_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/brackets-zed.scm");
pub(crate) const FOLDS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/folds.scm");
pub(crate) const HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/highlights.scm");
pub(crate) const INDENTS_ZED_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/indents-zed.scm");
pub(crate) const INDENTS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/indents.scm");
pub(crate) const INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/injections.scm");
pub(crate) const OUTLINE_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/outline.scm");
pub(crate) const OVERRIDES_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/overrides.scm");
pub(crate) const TAGS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/tags.scm");
pub(crate) const TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/cfml/queries/textobjects.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading CFML language");
    }
}
