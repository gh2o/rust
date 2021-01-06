//! Settings for tweaking completion.
//!
//! The fun thing here is `SnippetCap` -- this type can only be created in this
//! module, and we use to statically check that we only produce snippet
//! completions if we are allowed to.

use ide_db::helpers::{insert_use::MergeBehavior, SnippetCap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionConfig {
    pub enable_postfix_completions: bool,
    pub enable_autoimport_completions: bool,
    pub add_call_parenthesis: bool,
    pub add_call_argument_snippets: bool,
    pub snippet_cap: Option<SnippetCap>,
    pub merge: Option<MergeBehavior>,
}
