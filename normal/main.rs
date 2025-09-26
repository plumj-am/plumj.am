#![forbid(unsafe_code)]

mod blog;
mod contact;
mod home;
mod layout;
mod projects;

use blog::{
    BlogList,
    BlogPost,
};
use contact::Contact;
use dioxus::prelude::*;
use home::Home;
use layout::Layout;
use projects::Projects;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
	#[route("/")]
	Home {},
	#[route("/blog")]
	Blog {},
	#[route("/blog/:slug")]
	BlogPost { slug: String },
	#[route("/projects")]
	Projects {},
	#[route("/contact")]
	Contact {},
	#[route("/:..route")]
	PageNotFound { route: Vec<String> }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/gen-tailwind.css") }
        document::Link { rel: "icon", href: asset!("/favicon.ico") }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" }
        document::Script { src: "https://kit.fontawesome.com/6972f6e365.js", crossorigin: "anonymous", fetchpriority: "high" }
        div {
            Router::<Route> {}
        }
    }
}

#[component]
fn Blog() -> Element {
    rsx! { BlogList {} }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "404" }
    }
}
