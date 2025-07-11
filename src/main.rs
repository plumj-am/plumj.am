mod me;
mod projects;

use self::me::{ME, SOCIAL_LINKS};
use self::projects::PROJECTS;
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

/*----------------
  NAV COMPONENT
----------------*/
#[component]
fn Navbar() -> Element {
	rsx! {
		div {
			class: "flex flex-row gap-x-12 items-center",
			p {
				class: "border-1 border-white/20 p-1 rounded-sm",
				"jamesukiyo"
			}
			Link {
				class: "text-white decoration-none
					hover:cursor-pointer hover:text-[#ABABAB] hover:underline",
				to: Route::Home {},
				"Home"
			}
		}

		Outlet::<Route> {}
	}
}

//-------------------//
// PROFILE COMPONENT //
//-------------------//
#[component]
pub fn Profile() -> Element {
	rsx! {
			div {
				class: "m-0 flex flex-col justify-center align-center",
				img {
					class: "w-32 h-32 object-fit rounded-sm",
					src: "{ME.image}",
					alt: "{ME.name}"
				}
				h1 { "Name: {ME.name}" }
				p { "Age: {ME.age}" }
				p { "Languages:" }
				for l in ME.langs {
					p { "- {l}" }
				}
				p { "Frameworks:" }
				for f in ME.frameworks {
					p { "- {f}" }
				}

				div {
					class: "mt-4",
					img {
						src: "https://ghchart.rshah.org/000099/jamesukiyo",
						alt: "GitHub Contribution Chart",
						class: "w-xl"
					}
				}
				div {
					class: "mt-4",
					h1 { "Social Links" }
				div {
					class: "flex flex-row gap-x-4 items-center",
					for s in SOCIAL_LINKS {
						a {
							href: "{s.url}",
							target: "_blank",
							class: "flex items-center",
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

//----------------//
//   HOME PAGE    //
//----------------//
#[component]
fn Home() -> Element {
	rsx! {
		div {
			class: "m-0 flex flex-row justify-between mx-auto align-center max-w-6xl",
			Profile {}
			Projects {}
		}
	}
}

//------------------------//
// PROJECT LIST COMPONENT //
//------------------------//
#[component]
pub fn Projects() -> Element {
	rsx! {
		div {
			class: "text-left text-xl text-white flex flex-col flex-wrap",
			for p in PROJECTS {
				Link {
					class: "text-white mt-[20px] w-md
                        border-1 border-white rounded-md
                        p-4 hover:bg-[#1f1f1f] hover:cursor-pointer",
					to: "/projects/{p.name}",
					div {
					class: "flex flex-row justify-between border-b-1 border-white/20",
						span {
							class: "pb-2",
							"{p.name}"
						}
						span {
							class: "text-white/60",
							"[{p.type_of}]"
						}
					}
					span {
						class: "pt-2 text-sm text-white/60",
						"{p.tech_str()}"
					}
				}
			}
		}
	}
}

//----------------//
// 1 PROJECT PAGE //
//----------------//
#[component]
pub fn Project(name: String) -> Element {
	// Find the project info by name
	let p_info = PROJECTS.iter().find(|p| p.name == name);

	match p_info {
		Some(p) => rsx! {
			div {
				h1 { "name: {p.name}" }
				p { "description: {p.desc}" }
				p { "type of project: {p.type_of}" }
				p { "tech: {p.tech_str()}" }
				for t in p.tech {
					p { "- {t}" }
				}
			}
		},
		None => rsx! {
			div {
				h1 { "Project not found" }
				p { "The project '{name}' could not be found." }
			}
		},
	}
}
