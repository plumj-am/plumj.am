#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod data;
mod layout;
mod project;

use self::data::{ME, PROJECTS};
use self::layout::Layout;
use self::project::Project;
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

#[component]
fn LineNumbers() -> Element {
	let max_lines = 50;
	let line_numbers: Vec<i32> = (1..=max_lines).collect();
	rsx! {
		div { class: "pt-1 flex flex-col text-right text-white/40 pr-2 mr-3 border-r-1 border-white/20 select-none min-w-8",
			for line_num in line_numbers {
				div { class: "leading-6 h-6",
					"{line_num}"
				}
			}
		}
	}
}

// LEFT SIDE HOME PAGE
#[component]
pub fn Profile() -> Element {
	rsx! {
	div { class: "flex flex-col pt-1",
		div { class: "line-content",
			h1 { class: "border-b-1 border-white/20 w-fit",
				"Profile"
			}
		}
			div { class: "line-content", span { "" } }
			div { class: "flex flex-col items-start",
				img { class: "w-36 h-36 object-fit rounded-sm",
					src: "{ME.image}",
					alt: "{ME.name}"
				}
			}
			div { class: "line-content", span { "" } }
			div { class: "line-content", p { "Name: {ME.name}" } }
			div { class: "line-content", p { "Age: {ME.age}" } }
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
			div { class: "line-content", p { "Location: {ME.location}" } }
			div { class: "line-content", span { "" } }
			div { class: "grid grid-cols-2 gap-x-8",
				div { class: "flex flex-col",
					div { class: "line-content mb-[-1px]",
						h2 { "Languages:" }
					}
					for l in ME.langs {
						div { class: "line-content",
							p { "- {l}" }
						}
					}
				}
				div { class: "flex flex-col",
					div { class: "line-content mb-[-1px]",
						h2 { "Scripting:" }
					}
					for s in ME.scripting {
						div { class: "line-content",
							p { "- {s}" }
						}
					}
				}
			}
			div { class: "line-content", span { "" } }
			div { class: "grid grid-cols-2 gap-x-8",
				div { class: "flex flex-col",
					div { class: "line-content mb-[-1px]",
						h2 { "Frameworks:" }
					}
					for f in ME.frameworks {
						div { class: "line-content",
							p { "- {f}" }
						}
					}
				}
				div { class: "flex flex-col",
					div { class: "line-content mb-[-1px]",
						h2 { "Tools:" }
					}
					for t in ME.tools {
						div { class: "line-content",
							p { "- {t}" }
						}
					}
				}
			}

			div { class: "line-content", span { "" } }
			div { class: "flex flex-col",
				img { class: "max-w-md",
					src: "https://ghchart.rshah.org/000099/jamesukiyo",
					alt: "GitHub Contribution Chart",
				}
			}
			div { class: "line-content", span { "" } }
			div { class: "line-content flex flex-row gap-x-6 mt-1",
				for s in ME.socials {
					Link { class: "flex items-center hover:opacity-80",
						to: "{s.url}",
						new_tab: true,
						img { class: "max-w-5 max-h-5",
							src: s.icon,
							alt: "{s.name}",
						}
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
			div { class: "line-content", span { "" } }
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
									img { class: "w-5 h-5",
										src: asset!("/assets/github.svg"),
										alt: "GitHub",
									}
								}
							}
							if let Some(site_url) = p.site_url {
								Link { class: "opacity-80 hover:opacity-100",
									to: "{site_url}",
									new_tab: true,
									rel: "noopener noreferrer",
									img { class: "w-6 h-6",
										src: asset!("/assets/globe.svg"),
										alt: "Website",
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
