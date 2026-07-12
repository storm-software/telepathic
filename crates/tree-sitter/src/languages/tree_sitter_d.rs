//! D language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add the D language to a
//! tree-sitter [`Parser`][], and then use the parser to parse some code:
//!
//! ```
//! let code = "";
//! let mut parser = tree_sitter::Parser::new();
//! parser
//!     .set_language(&tree_sitter_d::LANGUAGE.into())
//!     .expect("Error loading D grammar");
//! let tree = parser.parse(code, None).unwrap();
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_d() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub(crate) const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_d) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub(crate) const NODE_TYPES: &str = include_str!("../../vendored/d/node-types.json");

pub(crate) const HELIX_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/helix-highlights.scm");
pub(crate) const HELIX_INDENTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/helix-indents.scm");
pub(crate) const HELIX_INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/helix-injections.scm");
pub(crate) const HELIX_TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/helix-textobjects.scm");
pub(crate) const HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/highlights.scm");
pub(crate) const INDENTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/indents.scm");
pub(crate) const INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/injections.scm");
pub(crate) const NOVA_FOLDS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/nova-folds.scm");
pub(crate) const NOVA_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/nova-highlights.scm");
pub(crate) const NOVA_SYMBOLS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/nova-symbols.scm");
pub(crate) const TAGS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/tags.scm");
pub(crate) const TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/d/queries/textobjects.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading D language");
    }
}
