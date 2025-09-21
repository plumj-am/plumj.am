use dioxus::prelude::*;

use super::Route;

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
    rsx! {
        div { class: "h-16 border-b-1 border-x-1 border-white/20 w-full max-w-5xl mx-auto",
            div { class: "grid grid-cols-4 w-full h-full text-2xl tracking-widest",
                div {
                    Link { class: "flex items-center justify-center w-full h-full border-r-1 border-white/20 hover:bg-purple text-base",
                        active_class: "bg-purple-light hover:cursor-default hover:bg-purple-light",
                        to: Route::Home {},
                        "HOME"
                    }
                }
                div {
                    Link { class: "flex items-center justify-center w-full h-full border-r-1 border-white/20 hover:bg-purple text-base",
                        active_class: "bg-purple-light hover:cursor-default hover:bg-purple-light",
                        to: Route::Projects {},
                        "PROJECTS"
                    }
                }
                div {
                    Link { class: "flex items-center justify-center w-full h-full border-r-1 border-white/20 hover:bg-purple text-base",
                        active_class: "bg-purple-light hover:cursor-default hover:bg-purple-light",
                        to: Route::Blog {},
                        "BLOG"
                    }
                }
                div {
                    Link { class: "flex items-center justify-center w-full h-full hover:bg-purple text-base",
                        active_class: "bg-purple-light hover:cursor-default hover:bg-purple-light",
                        to: Route::Contact {},
                        "CONTACT"
                    }
                }
            }

        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-8 border-t-1 border-x-1 border-white/20 w-full max-w-5xl mx-auto",
            p { class: "text-base/80 text-xs",
                "Copyright © 2025-present - "
                span { class: "text-base",
                    "PlumJam"
                }
                " <git@plumj.am>"
            }
        }
    }
}
