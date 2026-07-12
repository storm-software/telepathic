//! ObjectScript UDL language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add the ObjectScript UDL language to a
//! tree-sitter [`Parser`][], and then use the parser to parse some code:
//!
//! ```
//! let code = "";
//! let mut parser = tree_sitter::Parser::new();
//! parser
//!     .set_language(&tree_sitter_objectscript_udl::LANGUAGE.into())
//!     .expect("Error loading ObjectScript UDL grammar");
//! let tree = parser.parse(code, None).unwrap();
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_objectscript_udl() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub(crate) const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_objectscript_udl) };

pub(crate) const HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/objectscript_udl/queries/highlights.scm");
pub(crate) const INDENTS_SCM_QUERY: &str = include_str!("../../vendored/objectscript_udl/queries/indents.scm");
pub(crate) const INJECTIONS_SCM_QUERY: &str = include_str!("../../vendored/objectscript_udl/queries/injections.scm");
pub(crate) const STUDIO_HIGHLIGHTS_SCM_QUERY: &str = include_str!("../../vendored/objectscript_udl/queries/studio-highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading ObjectScript UDL language");
    }
}
