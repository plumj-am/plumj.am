mod data;
mod project;

use self::data::{ME, PROJECTS, VERSION};
use self::project::Project;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
	#[layout(Navbar)]
	#[route("/")]
	Home {},
	#[route("/project/:name")]
	Project { name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/gen-tailwind.css");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }
		Router::<Route> {}
	}
}

#[component]
fn Navbar() -> Element {
	rsx! {
		div {
			class: "flex flex-row justify-between items-center mb-8 max-w-5xl mx-auto",
			div {
				class: "flex flex-row gap-x-4 items-center",
				Link {
					class: "flex items-center hover:opacity-80",
					to: Route::Home {},
					img {
						src: asset!("assets/home.svg"),
						alt: "Home",
						class: "w-auto h-5"
					}
				}
			}
			div {
				class: "flex flex-row gap-x-4 items-center",
				a {
					class: "text-sm text-white/90 hover:opacity-80 hover:underline",
					href: "https://github.com/jamesukiyo/jamesukiyo.github.io/releases/tag/v{VERSION}",
					"v{VERSION}"
				}
				for s in ME.socials {
					a {
						href: "{s.url}",
						target: "_blank",
						class: "hover:opacity-80",
						img {
							src: "{s.icon}",
							alt: "{s.name}",
							class: "w-auto h-4"
						}
					}
				}
			}
		}

		Outlet::<Route> {}
	}
}

//
// HOME PAGE
//
#[component]
fn Home() -> Element {
	rsx! {
		div {
			class: "max-w-5xl mx-auto",
			div {
				class: "flex flex-col lg:flex-row justify-between align-center",
				Profile {}
				Projects {}
			}
			Footer {}
		}
	}
}

// LEFT SIDE HOME PAGE
#[component]
pub fn Profile() -> Element {
	rsx! {
		div {
			class: "max-h-[90vh] flex justify-start flex-col justify-center
				align-center gap-2",
			h1 { "Profile" }
			img {
				class: "w-32 h-32 object-fit rounded-sm mt-4",
				src: "{ME.image}",
				alt: "{ME.name}"
			}
			p { "Name: {ME.name}" }
			p { "Age: {ME.age}" }
			div {
				class: "flex items-center gap-2",
				p { "Email: {ME.email}" }
				button {
					class: "hover:cursor-pointer text-gray-500 hover:text-gray-700
						transition-colors",
					onclick: move |_| {
						let _ = document::eval(&format!(
								"navigator.clipboard.writeText('{}')",
								ME.email
							)
						);
					},
					title: "Copy email address",
					img {
						src: asset!("assets/copy.svg"),
						alt: "Copy",
						class: "w-4 h-4"
					}
				}
			}
			p { "Location: {ME.location}" }
			div {
				class: "grid grid-cols-2 gap-x-8 gap-y-4",
				div {
					h2 { "Languages:" }
					ul {
						for l in ME.langs {
							li { "• {l}" }
						}
					}
				}
				div {
					h2 { "Scripting:" }
					ul {
						for s in ME.scripting {
							li { "• {s}" }
						}
					}
				}
				div {
					h2 { "Frameworks:" }
					ul {
						for f in ME.frameworks {
							li { "• {f}" }
						}
					}
				}
				div {
					h2 { "Tools:" }
					ul {
						for t in ME.tools {
							li { "• {t}" }
						}
					}
				}
			}

			div {
				class: "mt-4 overflow-x-scroll md:overflow-x-visible flex sm:block justify-end",
				img {
					src: "https://ghchart.rshah.org/000099/jamesukiyo",
					alt: "GitHub Contribution Chart",
					class: "max-w-md"
				}
			}
			div {
				class: "mt-4",
				div {
					class: "flex flex-row gap-x-4 items-center",
					for s in ME.socials {
						a {
							href: "{s.url}",
							target: "_blank",
							class: "flex items-center hover:opacity-80",
							img {
								src: s.icon,
								alt: "{s.name}",
								class: "max-w-5 max-h-5"
							}
							p {
								class: "ml-2",
								"{s.name}"
							}
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
		div {
			class: "max-h-[80vh] text-xl lg:overflow-scroll no-scrollbar",
			h1 {
				class: "mt-8 lg:mt-0 lg:fixed bg-[#0f1116] w-full pb-2 z-10",
				"Projects ({PROJECTS.len()})"
			}
			div {
				class: "mt-12 text-left text-white flex flex-col gap-4",
				for p in PROJECTS {
					div {
						class: "text-white w-md border-1 border-white rounded-md
							pt-5 px-4 pb-3 hover:bg-[#1f1f1f] relative",
						Link {
							class: "block hover:cursor-pointer",
							to: "/project/{p.name}",
							div {
								class: "flex flex-row justify-between border-b-1
									border-white/20",
								span {
									class: "pb-2",
									"{p.name}"
								}
								span {
									class: "text-white/60 text-lg",
									"[{p.type_str()}]"
								}
							}
							div {
								class: "flex flex-row justify-between items-end",
								span {
									class: "pt-2 text-sm text-white/60",
									"{p.tech_str()}"
								}
							}
						}
						div {
							class: "absolute bottom-3 right-4 flex gap-2 items-center",
							if let Some(gh_url) = p.gh_url {
								a {
									href: "{gh_url}",
									target: "_blank",
									rel: "noopener noreferrer",
									class: "opacity-80 hover:opacity-100",
									img {
										src: asset!("/assets/github.svg"),
										alt: "GitHub",
										class: "w-5 h-5"
									}
								}
							}
							if let Some(site_url) = p.site_url {
								a {
									href: "{site_url}",
									target: "_blank",
									rel: "noopener noreferrer",
									class: "opacity-80 hover:opacity-100",
									img {
										src: asset!("/assets/globe.svg"),
										alt: "Website",
										class: "w-6 h-6"
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

#[component]
pub fn Footer() -> Element {
	rsx! {
		div {
			class: "absolute mt-auto bottom-0 font-sm text-white/20",
			p {
				"Copyright © 2025 - James Plummer <jamesp2001@live.co.uk>"
			}
		}
	}
}
