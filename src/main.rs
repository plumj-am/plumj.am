#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod data;
mod nerd;
mod normal;

use nerd::{Home as NerdHome, PageNotFound as NerdPageNotFound, Project as NerdProject};
use normal::{Home as NormalHome, PageNotFound as NormalPageNotFound};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[route("/")]
	Root {},

	#[nest("/normal")]
		#[layout(normal::Layout)]
		#[route("/")]
		NormalHome {},
		#[route("/:..route")]
		NormalPageNotFound { route: Vec<String>},
	#[end_nest]

	#[nest("/nerd")]
		#[layout(nerd::Layout)]
		#[route("/")]
		NerdHome {},
		#[route("/project/:name")]
		NerdProject { name: String },
		#[route("/:..route")]
		NerdPageNotFound { route: Vec<String> },
}

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
		document::Stylesheet { href: "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" }
		document::Script { src: "https://kit.fontawesome.com/6972f6e365.js", crossorigin: "anonymous", fetchpriority: "high" }
		div {
			Router::<Route> {}
		}
	}
}

#[component]
fn Root() -> Element {
	rsx! {
	document::Stylesheet { href: asset!("/assets/gen-normal-tailwind.css") }
		p { "hello" }
	}
}
