use super::Route;
use super::data::{ME, VERSION};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	rsx! {
		div {
			class: "min-h-screen flex flex-col min-w-full",
			Navbar {}
			main { class: "mt-6 flex flex-col grow max-w-5xl w-full mx-auto",
				Outlet::<Route> {}
			}
			Footer {}
		}
	}
}

#[component]
pub fn Navbar() -> Element {
	rsx! {
		div {
			class: "h-16 w-full flex flex-row justify-between items-center mb-8 max-w-5xl mx-auto",
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
				Link {
					class: "text-sm text-white/90 hover:opacity-80 hover:underline",
					to: "https://github.com/jamesukiyo/jamesukiyo.github.io/releases/tag/v{VERSION}",
					"v{VERSION}"
				}
				for s in ME.socials {
					Link {
						to: "{s.url}",
						new_tab: true,
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
	}
}

#[component]
pub fn Footer() -> Element {
	rsx! {
		div {
			class: "fixed bottom-0 font-sm text-white/20 bg-[#0f1116] w-full text-center",
			p {
				"Copyright © 2025 - James Plummer <jamesp2001@live.co.uk>"
			}
		}
	}
}
