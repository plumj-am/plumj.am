use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		div { class: "flex w-full",
			div { class: "flex flex-col lg:flex-row justify-between w-full",
				p { "This will be the normal home page."}
			}
		}
	}
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
	rsx! {
		div { class: "flex w-full",
			div { class: "flex flex-col lg:flex-row justify-between w-full",
				p { "This will be the normal not found page."}
			}
		}
	}
}
