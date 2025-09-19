use super::super::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	rsx! {
		document::Stylesheet { href: asset!("/assets/gen-normal-tailwind.css") }
		div { class: "min-h-screen flex flex-col min-w-full",
			Navbar {}
			main { class: "flex flex-col grow max-w-6xl w-full mx-auto bg-[#0f1116]",
				Outlet::<Route> {}
			}
			Footer {}
		}
	}
}

#[component]
pub fn Navbar() -> Element {
	rsx! {
		p { class: "text-center",
			"Navbar"
		}
	}
}

#[component]
pub fn Footer() -> Element {
	rsx! {
		p { class: "text-center",
			"Footer"
		}
	}
}
