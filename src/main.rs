#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod data;
mod layout;
mod project;
mod utils;

use self::data::{ME, PROJECTS};
use self::layout::Layout;
use self::project::Project;
use self::utils::{Line, LineNumbers, LineType};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[layout(Layout)]
	#[route("/")]
	Home {},
	#[route("/project/:name")]
	Project { name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/gen-tailwind.css");
const DEVICONS_CSS: &str =
	"https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css";

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }
		document::Link { rel: "stylesheet", href: DEVICONS_CSS }
		Router::<Route> {}
	}
}

//
// HOME PAGE
//
#[component]
fn Home() -> Element {
	rsx! {
		div { class: "flex w-full",
			LineNumbers {}
			div { class: "flex flex-col lg:flex-row justify-between w-full",
				Profile {}
				Projects {}
			}
		}
	}
}

// LEFT SIDE HOME PAGE
#[component]
pub fn Profile() -> Element {
	rsx! {
	div { class: "flex flex-col pt-1",
		Line { classes: "border-b-1 border-white/20 w-fit",
			type_of: LineType::H1,
			text: "Profile",
		}
			Line {}
			div { class: "flex flex-col items-start",
				img { class: "w-36 h-36 object-fit rounded-sm",
					src: "{ME.image}",
					alt: "{ME.name}"
				}
			}
			Line {}
			Line { type_of: LineType::P, text: "Name: {ME.name}" }
			Line { type_of: LineType::P, text: "Age: {ME.age}" }
			div { class: "line-content",
				p { "Email: {ME.email}" }
				button { class: "hover:cursor-pointer text-gray-500 hover:text-gray-700 transition-colors ml-2",
					onclick: move |_| {
						let _ = document::eval(&format!(
								"navigator.clipboard.writeText('{}')",
								ME.email
							)
						);
					},
					title: "Copy email address",
					img { class: "w-4 h-4",
						src: asset!("assets/copy.svg"),
						alt: "Copy",
					}
				}
			}
			Line { type_of: LineType::P, text: "Location: {ME.location}" }
			Line {}
			div { class: "grid grid-cols-2 gap-x-8",
				div { class: "flex flex-col",
					Line { classes: "mb-[-1px]",
						type_of: LineType::H2,
						text: "Languages:",
					}
					for l in ME.langs {
						Line { type_of: LineType::P, text: "- {l}" }
					}
				}
				div { class: "flex flex-col",
					Line { type_of: LineType::H2, text: "Scripting:" }
					for s in ME.scripting {
						Line { type_of: LineType::P, text: "- {s}" }
					}
				}
			}
			Line {}
			div { class: "grid grid-cols-2 gap-x-8",
				div { class: "flex flex-col",
					Line { classes: "mb-[-1px]",
						type_of: LineType::H2,
						text: "Frameworks:",
					}
					for f in ME.frameworks {
						Line { type_of: LineType::P, text: "- {f}" }
					}
				}
				div { class: "flex flex-col",
					Line { classes: "mb-[-1px]",
						type_of: LineType::H2,
						text: "Tools:",
					}
					for t in ME.tools {
						Line { type_of: LineType::P, text: "- {t}" }
					}
				}
			}

			Line {}
			div { class: "flex flex-col",
				img { class: "max-w-md",
					src: "https://ghchart.rshah.org/000099/jamesukiyo",
					alt: "GitHub Contribution Chart",
				}
			}
			Line {}
			div { class: "line-content flex flex-row gap-x-6 mt-1",
				for s in ME.socials {
					Link { class: "flex items-center hover:opacity-80",
						to: "{s.url}",
						new_tab: true,
						i { class: "{s.icon} text-white text-lg h-fit w-fit" }
						p { class: "ml-2",
							"{s.name}"
						}
					}
				}
			}
		}
	}
}

// RIGHT SIDE HOME PAGE
#[component]
pub fn Projects() -> Element {
	rsx! {
		div { class: "text-xl pt-1 mr-2",
			h1 { class: "border-b-1 border-white/20 w-fit",
				"Projects ({PROJECTS.len()})"
			}
			Line {}
			div { class: "text-left text-white flex flex-col gap-4",
				for p in PROJECTS {
					div { class: "text-white w-md border-1 border-white rounded-md
							pt-5 px-4 pb-3 hover:bg-[#1f1f1f] relative",
						Link { class: "block hover:cursor-pointer",
							to: Route::Project{ name: p.name.to_string() },
							div { class: "flex flex-row justify-between border-b-1 border-white/20",
								span { class: "pb-2",
									"{p.name}"
								}
								span { class: "text-white/60 text-lg",
									"[{p.type_of.as_str()}]"
								}
							}
							div { class: "flex flex-row justify-between items-end",
								span { class: "pt-2 text-sm text-white/60",
									"{p.tech_str()}"
								}
							}
						}
						div { class: "absolute bottom-3 right-4 flex gap-2 items-center",
							if let Some(gh_url) = p.gh_url {
								Link { class: "opacity-80 hover:opacity-100",
									to: "{gh_url}",
									new_tab: true,
									rel: "noopener noreferrer",
									i { class: "devicon-github-original text-white w-5 h-5" }
								}
							}
							if let Some(site_url) = p.site_url {
								Link { class: "opacity-80 hover:opacity-100",
									to: "{site_url}",
									new_tab: true,
									rel: "noopener noreferrer",
									i { class: "devicon-firefox-plain text-white w-6 h-6" }
								}
							}
						}
					}
				}
			}
		}
	}
}
