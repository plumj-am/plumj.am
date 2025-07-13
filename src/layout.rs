use super::Route;
use super::data::{ME, VERSION};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	rsx! {
		div { class: "min-h-screen flex flex-col min-w-full",
			Navbar {}
			main { class: "flex flex-col grow max-w-6xl w-full mx-auto px-1 border-x-1 border-t-1 border-white/20 bg-[#0f1116]",
				Outlet::<Route> {}
			}
			Footer {}
		}
	}
}

#[component]
pub fn Navbar() -> Element {
	rsx! {
		div { class: "h-12 bg-[#0f1116]"}
		div { class: "fixed top-0 z-99 left-0 right-0 bg-[#16161e] h-12 w-full flex flex-row justify-between items-center max-w-6xl px-3 mx-auto border-x-1 border-t-1 border-x-white/20 border-t-white/20",
			div { class: "flex flex-row gap-x-4 items-center",
				Link {
					class: "flex items-center hover:opacity-80",
					to: Route::Home {},
					img { class: "w-auto h-5",
						src: asset!("assets/home.svg"),
						alt: "Home",
					}
				}
			}
			div { class: "flex flex-row gap-x-4 items-center",
				Link { class: "text-sm text-white/90 hover:opacity-80 hover:underline",
					to: "https://github.com/jamesukiyo/jamesukiyo.github.io/releases/tag/v{VERSION}",
					"v{VERSION}"
				}
				for s in ME.socials {
					Link { class: "hover:opacity-80",
						to: "{s.url}",
						new_tab: true,
						img { class: "w-auto h-4",
							src: "{s.icon}",
							alt: "{s.name}",
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
		div { class: "fixed max-w-6xl bottom-0 left-0 right-0 bg-[#16161e] text-white
				text-md flex z-50 items-center mx-auto",
			// mode
			div { class: "bg-[#7aa2f7] text-[#16161e] px-3",
				"NORMAL"
			}
			// branch
			div { class: "bg-[#414868] text-[#c0caf5] px-3",
				span { class: "flex items-center",
					i { class: "devicon-git-plain mr-1" }
					"master"
				}
			}
			// file name
			div { class: "bg-[#24283b] text-[#c0caf5] px-3",
				"portfolio/main.rs"
			}
			// spacer
			div { class: "flex-grow" }
			// file type and position + copyright
			div {
				class: "flex items-center",
				div { class: "text-white/30 text-xs mr-2",
					"Copyright © 2025 - James Plummer <jamesp2001@live.co.uk>"
				}
				div { class: "bg-[#414868] text-[#c0caf5] px-3 flex items-center",
					span { class: "flex items-center",
						i { class: "devicon-rust-plain mr-1" }
					}
					"rust"
				}

				div { class: "bg-[#24283b] text-[#c0caf5] px-3",
					"42:12"
				}

				div { class: "bg-[#7aa2f7] text-[#16161e] px-3",
					"100%"
				}
			}
		}
	}
}
