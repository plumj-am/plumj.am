//! Markdown rendering for blog posts.

use pulldown_cmark::{
    Parser,
    html,
};
use topcoat::view::Unescaped;

/// Renders `CommonMark` to HTML, marked as trusted markup.
///
/// The input is the site's own committed post content, so escaping it again
/// would be wrong.
#[must_use]
pub fn to_html(markdown: &str) -> Unescaped<String> {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    Unescaped::new_unchecked(output)
}
