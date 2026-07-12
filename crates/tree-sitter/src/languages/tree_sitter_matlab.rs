//! Matlab language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add the Matlab language to a
//! tree-sitter [`Parser`][], and then use the parser to parse some code:
//!
//! ```
//! let code = "";
//! let mut parser = tree_sitter::Parser::new();
//! parser
//!     .set_language(&tree_sitter_matlab::LANGUAGE.into())
//!     .expect("Error loading Matlab grammar");
//! let tree = parser.parse(code, None).unwrap();
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_matlab() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub(crate) const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_matlab) };

pub(crate) const EMACS_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/emacs/highlights.scm");
pub(crate) const EMACS_TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/emacs/textobjects.scm");
pub(crate) const HELIX_CONTEXT_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/context.scm");
pub(crate) const HELIX_FOLDS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/folds.scm");
pub(crate) const HELIX_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/highlights.scm");
pub(crate) const HELIX_INDENTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/indents.scm");
pub(crate) const HELIX_INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/injections.scm");
pub(crate) const HELIX_LOCALS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/locals.scm");
pub(crate) const HELIX_TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/helix/textobjects.scm");
pub(crate) const NEOVIM_CONTEXT_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/context.scm");
pub(crate) const NEOVIM_FOLDS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/folds.scm");
pub(crate) const NEOVIM_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/highlights.scm");
pub(crate) const NEOVIM_INDENTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/indents.scm");
pub(crate) const NEOVIM_INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/injections.scm");
pub(crate) const NEOVIM_LOCALS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/locals.scm");
pub(crate) const NEOVIM_TAGS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/tags.scm");
pub(crate) const NEOVIM_TEXTOBJECTS_SCM_QUERY: &str = include_str!("../../vendored/matlab/queries/neovim/textobjects.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Matlab language");
    }
}
