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

fn parse_frontmatter(markdown: &str) -> Option<(serde_yaml::Value, &str)> {
    let markdown = markdown.strip_prefix("---")?;
    let (frontmatter, content) = markdown.split_once("\n---\n")?;
    let frontmatter = serde_yaml::from_str(frontmatter).ok()?;
    Some((frontmatter, content.trim()))
}

fn extract_code_blocks(content: &str) -> Vec<(&str, Option<&str>)> {
    let mut result = Vec::new();
    let mut pos = 0;

    while let Some(start) = content[pos..].find("```") {
        let block_start = pos.saturating_add(start);

        // Add text before code block.
        if block_start > pos {
            result.push((&content[pos..block_start], None));
        }

        let content_start = block_start.saturating_add(3);
        if let Some(end) = content[content_start..].find("```") {
            let block_end = content_start.saturating_add(end);
            let block_content = &content[content_start..block_end];

            if let Some(newline) = block_content.find('\n') {
                let lang = block_content[..newline].trim();
                let code = &block_content[newline.saturating_add(1)..];
                result.push((code, (!lang.is_empty()).then_some(lang)));
            } else {
                result.push((block_content, None));
            }
            pos = block_end.saturating_add(3);
        } else {
            result.push((&content[block_start..], None));
            break;
        }
    }

    if pos < content.len() {
        result.push((&content[pos..], None));
    }

    result
}

#[component]
fn MarkdownContent(content: String) -> Element {
    rsx! {
        { extract_code_blocks(&content).into_iter().filter_map(|(text, lang)| {
            if text.trim().is_empty() { return None; }

            Some(match lang {
                // Create a simple, bordered code block.
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
                // All other text as `p` tag.
                None => rsx! {
                    p { class: "whitespace-pre-wrap text-md text-fg mb-2",
                        "{text}"
                    }
                }
            })
        })}
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
                                        "#{(i + 1).to_string()}"
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
                                "#{index + 1}"
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
