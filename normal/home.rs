use common::data::{
    ME,
    PROJECTS,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex w-full flex-col lg:flex-row gap-8 py-6",
            Profile {}
            Projects {}
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 lg:w-1/2",
            div { class: "text-center lg:text-left",
                img { class: "w-32 h-32 mx-auto lg:mx-0 mb-4",
                    src: "{ME.image_url}",
                    alt: "{ME.name}"
                }
                h1 { class: "text-3xl font-bold mb-2 text-fg",
                    "{ME.name}"
                }
                p { class: "text-fg opacity-70",
                    a { class: "hover:opacity-100 transition-opacity",
                        href: "mailto:{ME.email}",
                        "{ME.email}"
                    }
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    h3 { class: "font-semibold mb-2 text-fg", "Languages" }
                    ul { class: "text-sm text-fg opacity-80 space-y-1",
                        for lang in ME.langs {
                            li { "• {lang}" }
                        }
                    }
                }
                div {
                    h3 { class: "font-semibold mb-2 text-fg", "Frameworks" }
                    ul { class: "text-sm text-fg opacity-80 space-y-1",
                        for framework in ME.frameworks.iter().take(4) {
                            li { "• {framework}" }
                        }
                    }
                }
            }

            div { class: "flex gap-4 justify-center lg:justify-start",
                for social in ME.socials {
                    a { class: "group flex items-center text-fg hover:text-purple-dark transition-all duration-100 overflow-hidden p-1",
                        href: "{social.url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        i { class: "{social.icon} text-xl" }
                        span { class: "ml-1 max-w-0 group-hover:max-w-xs transition-all duration-100 whitespace-nowrap opacity-0 group-hover:opacity-100",
                            "{social.name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 lg:w-1/2",
            h2 { class: "text-2xl font-bold text-fg",
                "Projects (6/{PROJECTS.len()})"
            }

            div { class: "grid gap-4",
                for project in PROJECTS.iter().take(6) {
                    div { class: "border border-fg p-4 hover:bg-[var(--color-purple-light)] transition-scale hover:scale-110 duration-100",
                        div { class: "flex justify-between items-start mb-2",
                            h3 { class: "font-semibold text-[var(--color-fg)]", "{project.name}" }
                            span { class: "text-xs px-2 py-1 bg-green-light text-fg",
                                "{project.project_type.as_str()}"
                            }
                        }
                        p { class: "text-sm text-fg opacity-80 mb-3",
                            "{project.short_desc}"
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-xs text-fg opacity-60",
                                "{project.tech_used_str()}"
                            }
                            div { class: "flex gap-2",
                                if let Some(github_url) = project.github_url() {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{github_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa-brands fa-github" }
                                    }
                                }
                                if let Some(site_url) = project.site_url {
                                    a { class: "text-fg hover:opacity-70",
                                        href: "{site_url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        i { class: "fa fa-globe" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
