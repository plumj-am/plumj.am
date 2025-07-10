use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
	#[layout(Navbar)]
	#[route("/")]
	Home {},
	#[route("/projects")]
	Projects,
	#[route("/projects/:name")]
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
			class: "flex flex-row",
			Link {
				class: "text-white mr-[20px] decoration-none
					hover:cursor-pointer hover:text-[#91a4d2]",
				to: Route::Home {},
				"Home"
			}
			Link {
				class: "text-white mr-[20px] decoration-none
					hover:cursor-pointer hover:text-[#91a4d2]",
				to: Route::Projects {},
				"Projects"
			}
		}

		Outlet::<Route> {}
	}
}

//----------------//
// HERO COMPONENT //
//----------------//
#[component]
pub fn Hero() -> Element {
	rsx! {
		div {
			id: "hero",
			class: "m-0 flex flex-col justify-center align-center",
			div {
				class: "w-[400px] text-left text-xl text-white flex flex-col",
					a {
						href: "/projects/dr-radka",
						"Dr Radka [website]"
					}
					a {
						href: "/projects/charfreq-rs",
						"charfreq-rs [CLI tool]"
					}
					a {
						href: "/projects/shell-rs",
						"shell-rs [CLI tool]"
					}
					a {
						href: "/projects/windows-setup",
						"windows-setup [script]"
					}
					a {
						href: "/projects/search-this",
						"search-this.nvim [neovim plugin]"
					}
					a {
						href: "/projects/server-health-monitor",
						"server-health-monitor [utility]"
					}
					a {
						href: "/projects/pausarr",
						"pausarr [script]"
					}
					a {
						href: "/projects/nvim",
						"nvim [config]"
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
		Hero {}

	}
}

//----------------//
// PROJECTS PAGE  //
//----------------//
#[component]
pub fn Projects() -> Element {
	rsx! {
		div {
			class: "mt-[50px]",

			// Content
			h1 { "This is the projects page" }
			p { "..." }
		}
	}
}

//----------------//
// 1 PROJECT PAGE //
//----------------//
#[component]
pub fn Project(name: String) -> Element {
	rsx! {
		div {
			id: "project",

			// Content
			h1 { "This is project #{name}!" }
			p { "In project #{name}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
		}
	}
}
