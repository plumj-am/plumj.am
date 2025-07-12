use super::Route;
use super::data::{PROJECTS, ProjectInfo};
use dioxus::prelude::*;

fn find_project(name: &str) -> Option<&'static ProjectInfo> {
	PROJECTS.iter().find(|p| p.name == name)
}

#[component]
pub fn Project(name: String) -> Element {
	match find_project(&name) {
		Some(p) => rsx! {
			div {
				class: "max-w-5xl mx-auto",
				h1 { "name: {p.name}" }
				p { "description: {p.desc}" }
				p { "type of project: {p.type_str()}" }
				p { "tech: {p.tech_str()}" }

				p { "GitHub: {p.gh_url_str()}" }
				p { "Website: {p.site_url_str()}" }

				h2 { "Technologies used:" }
				ul {
					for t in p.tech {
						li { "• {t}" }
					}
				}
			}
		},
		None => rsx! {
			div {
				class: "max-w-5xl mx-auto",
				h1 { "Project not found" }
				p { "Project '{name}' not found" }
				Link {
					to: Route::Home {},
					class: "text-white decoration-none hover:cursor-pointer
						hover:opacity-80 hover:underline",
					"← Back to Home"
				}
			}
		},
	}
}
