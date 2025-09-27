use std::sync::OnceLock;

use common::data::{
    LOGO_NO_BG,
    ME,
};
use dioxus::prelude::*;

use super::Route;

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

// TODO: I would like to improve the parsing capabilities. Not to allow more
// formatting options but just to make the process better/faster.

// TODO: Handle missing details. Currently it causes all blogs to disappear.
/// Extract blog details from frontmatter in `yaml` format.
fn parse_frontmatter(markdown: &str) -> Option<(serde_yaml::Value, &str)> {
    let markdown = markdown.strip_prefix("---")?;
    let (frontmatter, content) = markdown.split_once("\n---\n")?;
    let frontmatter = serde_yaml::from_str(frontmatter).ok()?;
    Some((frontmatter, content.trim()))
}

/// Extract code blocks from markdown content.
/// Returns a vector of `(content, language)` tuples for code blocks found.
fn parse_code_blocks(content: &str) -> Vec<(usize, usize, &str, Option<&str>)> {
    let mut blocks = Vec::new();
    let mut pos = 0;

    // Find all ``` code blocks in the content.
    while let Some(start) = content[pos..].find("```") {
        let block_start = pos.saturating_add(start);
        let content_start = block_start.saturating_add(3); // Skip opening ```.

        // Look for closing ```.
        if let Some(end) = content[content_start..].find("```") {
            let block_end = content_start.saturating_add(end);
            let block_content = &content[content_start..block_end];

            // Check if first line specificies a language.
            if let Some(newline) = block_content.find('\n') {
                let lang = block_content[..newline].trim();
                let code = &block_content[newline.saturating_add(1)..];
                blocks.push((
                    block_start,
                    block_end.saturating_add(3), // Include closing ```.
                    code,
                    (!lang.is_empty()).then_some(lang),
                ));
            } else {
                // No newline, treat entire content as code with no language.
                blocks.push((
                    block_start,
                    block_end.saturating_add(3),
                    block_content,
                    None,
                ));
            }
            pos = block_end.saturating_add(3); // Continue after closing ```.
        } else {
            // No closing ```, stop parsing.
            break;
        }
    }

    blocks
}

/// Extract image references from markdown content.
/// Returns a vector of `(start, end, src)` tuples for images found using
/// `[[["path"]]]` syntax.
fn parse_images(content: &str) -> Vec<(usize, usize, &str)> {
    let mut images = Vec::new();
    let mut pos = 0;

    // Find all `[[["path"]]]` entries in the content.
    while let Some(start) = content[pos..].find("[[[\"") {
        let img_start = pos.saturating_add(start);
        let src_start = img_start.saturating_add(4); // Skip opening `[[["`.

        // Look for closing `"]]]`.
        if let Some(end) = content[src_start..].find("\"]]]") {
            let src_end = src_start.saturating_add(end);
            let img_end = src_end.saturating_add(4); // Include closing `"]]]`.
            let src = &content[src_start..src_end]; // Extract image path.
            images.push((img_start, img_end, src));
            pos = img_end; // Continue after closing `"]]]`.
        } else {
            // No closing `"]]]`, skip this opening marker and continue.
            pos = src_start;
        }
    }

    images
}

#[component]
fn MarkdownContent(content: String) -> Element {
    let code_blocks = parse_code_blocks(&content);
    let images = parse_images(&content);

    // Create a vector of all content segments with their positions.
    let mut segments = Vec::new();

    // Add code blocks.
    for (start, end, code, lang) in code_blocks {
        segments.push((start, end, "code", code, lang));
    }

    // Add images.
    for (start, end, src) in images {
        segments.push((start, end, "image", src, None));
    }

    // Sort by position.
    segments.sort_by_key(|&(start, ..)| start);

    // Split content into rendered segments.
    let mut elements = Vec::new();
    let mut last_pos = 0;

    for (start, end, block_type, text, lang) in segments {
        // Add text before this block.
        if start > last_pos {
            let text_content = &content[last_pos..start];
            if !text_content.trim().is_empty() {
                elements.push(rsx! {
                    p { class: "whitespace-pre-wrap text-md text-fg mb-2",
                        "{text_content}"
                    }
                });
            }
        }

        // Add the block itself.
        match block_type {
            "code" => {
                elements.push(match lang {
                    // Create code block with language displayed.
                    Some(language) => rsx! {
                        div { class: "mb-2",
                            div { class: "bg-bg border border-purple border-b-0 px-3 py-1 text-md text-purple-light/70 font-mono",
                                "{language}"
                            }
                            p { class: "whitespace-pre-wrap bg-bg text-purple-light p-4 overflow-x-auto border border-purple font-mono text-md m-0",
                                "{text}"
                            }
                        }
                    },
                    // All other text as `p` tags.
                    None => rsx! {
                        p { class: "whitespace-pre-wrap bg-bg text-purple-light p-4 overflow-x-auto border border-purple font-mono text-md m-0 mb-2",
                            "{text}"
                        }
                    }
                });
            },
            "image" => {
                elements.push(rsx! {
                    div { class: "mb-2 py-2",
                        img {
                            class: "max-w-3/4 max-h-96 h-auto",
                            src: "{text}",
                            alt: "{text}"
                        }
                    }
                });
            },
            _ => {},
        }

        last_pos = end;
    }

    // Add any remaining text after markers in bulk.
    if last_pos < content.len() {
        let remaining = &content[last_pos..];
        if !remaining.trim().is_empty() {
            elements.push(rsx! {
                p { class: "whitespace-pre-wrap text-md text-fg mb-2",
                    "{remaining}"
                }
            });
        }
    }

    rsx! {
        { elements.into_iter() }
    }
}

#[rustfmt::skip]
static POSTS: &[&str] = &[
    include_str!("posts/is-this-thing-on.md")
];

fn posts() -> &'static [Post] {
    static PARSED: OnceLock<Vec<Post>> = OnceLock::new();
    PARSED.get_or_init(|| {
        let mut posts: Vec<_> = POSTS
            .iter()
            .filter_map(|&md| Post::from_markdown(md))
            .collect();
        posts.sort_unstable_by(|a, b| b.date.cmp(&a.date));
        posts
    })
}

fn find_post_with_index(slug: &str) -> Option<(&'static Post, usize)> {
    posts()
        .iter()
        .enumerate()
        .find(|&(_, p)| p.slug == slug)
        .map(|(i, p)| (p, i))
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div { class: "w-full py-8",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 px-4 lg:px-0",
                for (i, post) in posts().iter().enumerate() {
                    Link {
                        to: Route::BlogPost { slug: post.slug.clone() },
                        div { class: "border border-fg p-4 hover:bg-purple-light transition-scale hover:scale-105 lg:hover:scale-110 duration-100 hover:z-10",
                            div { class: "mb-1",
                                div { class: "flex justify-between",
                                    h1 { class: "text-xl font-semibold text-fg mb-3",
                                        "{post.title}"
                                    }
                                    p { class: "text-fg opacity-60",
                                        "#{(posts().len() - i).to_string()}"
                                    }
                                }
                                p { class: "text-fg opacity-70 mb-3",
                                    "{post.brief}"
                                }
                                p { class: "text-fg opacity-60",
                                    "{post.date}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    match find_post_with_index(&slug) {
        Some((post, index)) => {
            rsx! {
                div { class: "flex flex-col py-6",
                    div { class: "border border-fg p-4 relative",
                        div { class: "flex justify-between items-start",
                            h1 { class: "text-3xl font-bold text-fg mb-3",
                                "{post.title}"
                            }
                            p { class: "text-lg text-fg opacity-60",
                                "#{posts().len() - index}"
                            }
                        }
                        div { class: "text-md text-fg flex items-end justify-between",
                            p { class: "opacity-60",
                                "{post.date} "
                                span { class: "text-xs opacity-40",
                                    "(last edit: {post.edited})"
                                }
                            }
                            span { class: "group flex items-center",
                                img { class: "w-4 mr-2 group-hover:scale-400 group-hover:translate-x-[-30px] transition-all duration-300 animate-bounce",
                                    src: LOGO_NO_BG,
                                    alt: "{ME.name}"
                                }
                                span { class: "text-purple-light",
                                    "{post.author}"
                                }
                            }
                        }
                    }
                    article { class: "border-x border-fg px-4 py-6",
                        MarkdownContent { content: &post.content }
                        Credits {}
                    }
                    div { class: "group border border-fg p-4 hover:bg-purple-light hover:scale-105 transition-all duration-100",
                        Link { class: "text-fg",
                            to: Route::Blog {},
                            "← Back to the blog list."
                        }
                    }
                }
            }
        },
        None => {
            rsx! {
                div { class: "flex flex-col py-6",
                    div { class: "border border-fg p-4 relative",
                        div { class: "flex justify-between items-start",
                            h1 { class: "text-3xl font-bold text-fg mb-3",
                                "Post Not Found :("
                            }
                        }
                    }
                    article { class: "border-x border-fg px-4 py-6",
                        "This post doesn't exist..."
                    }
                    div { class: "group border border-fg p-4 hover:bg-purple-light hover:scale-105 transition-all duration-100",
                        Link { class: "text-fg",
                            to: Route::Blog {},
                            "← Back to the blog list."
                        }
                    }
                }
            }
        },
    }
}

#[component]
fn Credits() -> Element {
    rsx! {
        div {class: "mt-8 w-full",
        hr { class: "opacity-20" }
            p { class: "mt-8 mb-4 flex flex-col",
                "You can find me on..."
                // TODO: I really need to stop repeating this everywhere.
                p { class: "mt-2",
                    "Xitter: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://x.com/plumj_am",
                        "@plumj_am"
                    }
                }
                p { "GitHub: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://github.com/plumj-am",
                        "plumj-am"
                    }
                }
                p { "Forgejo:"
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://git.plumj.am/plumjam",
                        "plumjam"
                    }
                }
                p { "Matrix: "
                    Link { class: "ml-2 hover:text-purple-light opacity-80 hover:opacity-100",
                        to: "https://matrix.to/#/@plumjam:plumj.am",
                        "@plumjam:plumj.am"
                    }
                }
            }
        }
    }
}
