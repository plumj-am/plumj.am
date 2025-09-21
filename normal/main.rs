#![forbid(unsafe_code)]

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[route("/")]
	Home {},
	#[route("/:..route")]
	PageNotFound { route: Vec<String> }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("gen-tailwind.css") }
        document::Link { rel: "icon", href: asset!("favicon.ico") }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" }
        document::Script { src: "https://kit.fontawesome.com/6972f6e365.js", crossorigin: "anonymous", fetchpriority: "high" }
        div {
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Link { rel: "icon", href: "/favicon.ico" }
        h1 { "Normal App" }
        p { "This is the normal version of plumj.am" }
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "404" }
    }
}
