use super::Route;
use super::data::{ME, PROJECTS, VERSION};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	rsx! {
		div { class: "min-h-screen flex flex-col min-w-full",
			Navbar {}
			main { class: "flex flex-col grow max-w-6xl w-full mx-auto border-x-1 border-t-1 border-white/20 bg-[#0f1116]",
				Outlet::<Route> {}
			}
			Footer {}
		}
	}
}

struct Keymap {
	key: &'static str,
	action: &'static str,
}

const MAPPINGS: &[Keymap] = &[
	Keymap {
		key: "gg",
		action: "top",
	},
	Keymap {
		key: "G",
		action: "bottom",
	},
	Keymap {
		key: "d",
		action: "½ down",
	},
	Keymap {
		key: "u",
		action: "½ up",
	},
	Keymap {
		key: "j",
		action: "line down",
	},
	Keymap {
		key: "k",
		action: "line up",
	},
];

#[component]
pub fn Navbar() -> Element {
	rsx! {
		div { class: "h-12 bg-[#0f1116]"}
		div { class: "fixed top-0 z-99 left-0 right-0 bg-[#16161e] h-12 w-full flex flex-row items-center max-w-6xl px-3 mx-auto border-x-1 border-t-1 border-x-white/20 border-t-white/20",
			// left col
			div { class: "flex items-center",
				Link {
					class: "flex items-center hover:opacity-80",
					to: Route::Home {},
					i { class: "fa-solid fa-house" }
				}
			}
			// center col
			div { class: "flex-1 flex justify-center",
				div { class: "flex items-center gap-x-3 text-sm",
					i { class: "devicon-vim-plain text-2xl" }
					div { class: "flex items-center gap-x-4",
						for k in MAPPINGS {
							div { class: "flex items-center gap-x-2",
								kbd { class: "bg-gray-700 px-1 rounded text-md", "{k.key}" }
								span { class: "text-sm! font-mono", "{k.action}" }
							}
						}
					}
				}
			}
			// right col
			div { class: "flex flex-row gap-x-4 items-center",
				Link { class: "text-sm text-white/90 hover:opacity-80 hover:underline",
					to: "https://github.com/jamesukiyo/jamesukiyo.github.io/releases/tag/v{VERSION}",
					"v{VERSION}"
				}
				for s in ME.socials {
					Link { class: "hover:opacity-80",
						to: "{s.url}",
						new_tab: true,
						i { class: "{s.icon}" }
					}
				}
			}
		}
	}
}

#[component]
pub fn Footer() -> Element {
	let curr_route = use_route::<Route>();
	let (filename, filetype) = match curr_route {
		Route::Home {} => ("portfolio/main.rs".to_string(), "rust".to_string()),
		Route::Project { name } => {
			let tech = PROJECTS
				.iter()
				.find(|p| p.name == name)
				.map_or("rust", |p| p.main_tech())
				.to_lowercase();

			let ext = match tech.as_str() {
				"rust" => "rs",
				"c++" => "cpp",
				"sveltekit" => "svelte",
				_ => &tech,
			};

			// strip project name extensions and use underscores for file name
			let clean_name = name
				.replace(['.', '-'], "_")
				.replace("_nvim", "")
				.replace("_rs", "")
				.replace("_github_io", "");

			(format!("portfolio/{clean_name}.{ext}"), tech)
		}
	};

	let filetype_component = {
		let (icon, display) = match filetype.as_str() {
			"lua" => ("lua", "lua"),
			"bash" => ("bash", "bash"),
			"c++" => ("cplusplus", "cpp"),
			"sveltekit" | "svelte" => ("svelte", "svelte"),
			_ => ("rust", "rust"),
		};

		rsx! {
			span { class: "flex items-center",
				i { class: "devicon-{icon}-plain mr-1" }
				"{display}"
			}
		}
	};

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
				"{filename}"
			}
			// spacer
			div { class: "flex-grow" }
			// file type and position + copyright
			div { class: "flex items-center",
				div { class: "text-white/30 text-xs mr-2",
					"Copyright © 2025 - James Plummer <jamesp2001@live.co.uk>"
				}
				div { class: "bg-[#414868] text-[#c0caf5] px-3 flex items-center",
					{filetype_component}
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
