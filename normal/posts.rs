//! Blog posts parsed from the `posts` directory.

use std::sync::LazyLock;

use libs::data::ME;
use serde_yaml_bw::Value;

/// A parsed blog post.
#[derive(Clone)]
pub struct Post {
    pub title:   String,
    pub date:    String,
    pub edited:  String,
    pub slug:    String,
    pub brief:   String,
    pub author:  String,
    pub content: String,
}

const SOURCES: &[&str] = &[
    include_str!("posts/is-this-thing-on.md"),
    include_str!("posts/maybe-nix-does-fix-everything.md"),
    include_str!("posts/vim-helix-zed-helix-again.md"),
];

impl Post {
    fn from_markdown(markdown: &str) -> Option<Self> {
        let (frontmatter, content) = parse_frontmatter(markdown)?;
        Some(Self {
            title:   frontmatter.get("title")?.as_str()?.to_owned(),
            date:    frontmatter.get("date")?.as_str()?.to_owned(),
            edited:  frontmatter.get("edited")?.as_str()?.to_owned(),
            slug:    frontmatter.get("slug")?.as_str()?.to_owned(),
            brief:   frontmatter.get("brief")?.as_str()?.to_owned(),
            author:  String::from(ME.name),
            content: content.to_owned(),
        })
    }
}

/// Extracts the YAML frontmatter and the Markdown body.
fn parse_frontmatter(markdown: &str) -> Option<(Value, &str)> {
    let markdown = markdown.strip_prefix("---")?;
    let (frontmatter, content) = markdown.split_once("\n---\n")?;
    let frontmatter = serde_yaml_bw::from_str(frontmatter).ok()?;
    Some((frontmatter, content.trim()))
}

/// Every post, newest first.
#[must_use]
pub fn all() -> &'static [Post] {
    &POSTS
}

static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| {
    let mut posts: Vec<_> = SOURCES
        .iter()
        .filter_map(|markdown| Post::from_markdown(markdown))
        .collect();
    posts.sort_unstable_by(|left, right| right.date.cmp(&left.date));
    posts
});

/// Finds a post by slug, with its index in the newest-first list.
#[must_use]
pub fn find(slug: &str) -> Option<(&'static Post, usize)> {
    all()
        .iter()
        .enumerate()
        .find(|entry| entry.1.slug == slug)
        .map(|(index, post)| (post, index))
}
