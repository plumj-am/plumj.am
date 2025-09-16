use super::Route;
use super::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn Project(name: String) -> Element {
	// for me later: first closure handles None, second closure handles Some
	PROJECTS.iter().find(|p| p.name == name).map_or_else(
		|| {
			rsx! {
				div { class: "max-w-5xl mx-auto",
					h1 { "Project not found" }
					p { "Project '{name}' not found" }
					Link {
						to: Route::Home {},
						class: "text-white decoration-none hover:cursor-pointer
						hover:opacity-80 hover:underline",
						"← Back to Home"
					}
				}
			}
		},
		|p| {
			rsx! {
				div { class: "max-w-5xl mx-auto w-full",
					div { class: "flex flex-col",
						h1 { "name: {p.name}" }
						p { "description: {p.long_desc}" }
						p { "type: {p.project_type.as_str()}" }
						span {"GitHub: "
							Link {
								to: "{p.github_url_str()}",
								span { "{p.github_url_str()}" }
							}
						}
						span {"Website: "
							Link {
								to: "{p.site_url_str()}",
								span { "{p.site_url_str()}" }
							}
						}
						h2 { "Technologies used:" }
						ul {
							for t in p.tech_used {
								li { "• {t}" }
							}
						}
					}
				}
			}
		},
	)
}
