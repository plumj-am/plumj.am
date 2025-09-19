use super::super::Route;
use super::super::data::PROJECTS;
use super::utils::{HiCursor, HiLine, Line, LineNumbers};
use dioxus::prelude::*;

#[component]
pub fn Project(name: String) -> Element {
	// for me later: first closure handles None, second closure handles Some
	PROJECTS
		.iter()
		.find(|p| p.clean_name() == name)
		.map_or_else(
			|| {
				rsx! {
					div { class: "flex w-full",
						HiLine {}
						LineNumbers { max_lines: 70 }
						div { class: "flex flex-col w-full",
							HiCursor {}
							h1 { "Project not found" }
							Line {}
							p { "Project '{name}' not found" }
							Link {
								to: Route::NerdHome {},
								class: "text-white decoration-none hover:cursor-pointer
								hover:opacity-80 hover:underline",
								"← Back to Home"
							}
						}
					}
				}
			},
			|p| {
				rsx! {
					div { class: "flex w-full",
						HiLine {}
						LineNumbers { max_lines: 70 }
						div { class: "flex flex-col w-full",
							HiCursor {}
							h1 { "Name: {p.name}" }
							Line {}
							p { "Description: {p.long_desc}" }
							Line {}
							p { "Type: {p.project_type.as_str()}" }
							Line {}
							span {
								i { class: "fa-brands fa-github mr-1" }
								"GitHub: "
								if let Some(github_url) = p.github_url() {
									Link { class: "text-white/90 hover:underline hover:text-[#A66AA2]",
										to: "{github_url}",
										new_tab: true,
										span { ""{github_url.replace("https://",  " ")}"" }
									}
								} else {
									span { class: "opacity-40",
										"Currently private."
									}
								}
							}
							span {
								i { class: "fa-brands fa-git-alt mr-1" }
								"Forgejo: "
								if let Some(forgejo_url) = p.forgejo_url() {
									Link { class: "text-white/90 hover:underline hover:text-[#A66AA2]",
										to: "{forgejo_url}",
										new_tab: true,
										span { ""{forgejo_url.replace("https://",  " ")}"" }
									}
								} else {
									span { class: "opacity-40",
										"Currently private."
									}
								}
							}
							if let Some(npm_url) = p.npm_url {
								span {
									i { class: "fa-brands fa-npm mr-1" }
									"NPM: "
									Link { class: "text-white/90 hover:underline hover:text-[#A66AA2]",
										to: "{npm_url}",
										new_tab: true,
										span { ""{npm_url.replace("https://www.",  " ")}"" }
									}
								}
							}
							if let Some(crate_url) = p.crate_url {
								span {
									i { class: "fa-brands fa-rust mr-1" }
									"Crate: "
									Link { class: "text-white/90 hover:underline hover:text-[#A66AA2]",
										to: "{crate_url}",
										new_tab: true,
										span { ""{crate_url.replace("https://",  " ")}"" }
									}
								}
							}
							if let Some(site_url) = p.site_url {
								span {
									i { class: "fa fa-globe mr-1" }
									"Site: "
									Link { class: "text-white/90 hover:underline hover:text-[#A66AA2]",
										to: "{site_url}",
										new_tab: true,
										span {
										""{site_url.replace("https://",  " ")}""
											if p.name == "plumj.am" {
												span { class: "opacity-40",
													" (You're already here :)"
												}
											}
										}
									}
								}
							}
							Line {}
							h2 { "Technologies used:" }
							ul {
								for t in p.tech_used {
									li { "• {t}" }
								}
							}
							Line {}
							if p.media.is_some() {
								h2 { "Media:" }
								// TODO: media here.
							}
						}
					}
				}
			},
		)
}
