use common::data::LOGO_NO_BG;
use dioxus::prelude::*;

use super::Route;

struct NavLink {
    route: Route,
    text:  &'static str,
}

static NAV_LINKS: &[NavLink] = &[
    NavLink {
        route: Route::Home {},
        text:  "HOME",
    },
    NavLink {
        route: Route::Projects {},
        text:  "PROJECTS",
    },
    NavLink {
        route: Route::Blog {},
        text:  "BLOG",
    },
    NavLink {
        route: Route::Contact {},
        text:  "CONTACT",
    },
];

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col min-w-full",
            Navbar {}
            main { class: "flex flex-col grow max-w-5xl w-full mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

#[component]
fn Navbar() -> Element {
    let curr_route = use_route::<Route>();

    rsx! {
        div { class: "h-16 border-l-1 border-b-1 border-fg w-full max-w-5xl mx-auto",
            div { class: "grid grid-cols-4 w-full h-full text-2xl tracking",
                for link in NAV_LINKS {
                    div {
                        Link { class: "group flex items-center justify-center w-full h-full border-fg hover:bg-[var(--color-hover)] transition-scale duration-100 hover:scale-120 hover:shadow-[inset_0_0_0_1px_var(--color-fg)] hover:border-0 border-r-1",
                            active_class: "text-[var(--color-fg)] bg-[var(--color-active)] hover:bg-[var(--color-active)] hover:cursor-default text-base-light border-fg border-r-1",
                            to: link.route.clone(),
                            p { class: if curr_route == link.route { "scale-150" } else { "group-hover:scale-150 transition-scale duration-100" },
                                "{link.text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "group flex items-center justify-center h-8 border-t-1 border-x-1 border-fg w-full max-w-5xl mx-auto",
            p { class: "text-xs flex",
                span { class: "group-hover:translate-x-[-20px] transition-translate duration-300",
                    "Copyright © 2025-present"
                }
                img { class: "h-4 px-2 group-hover:scale-500 group-hover:translate-y-[-40px] transition-translate-y duration-300 animate-bounce",
                    src: LOGO_NO_BG
                }
                span { class: "group-hover:translate-x-[20px] transition-translate duration-300",
                    span { class: "text-purple-light pr-2",
                        "PlumJam"
                    }
                "<git@plumj.am>"
                }
            }
        }
    }
}
