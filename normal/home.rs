use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex w-full",
            div { class: "flex flex-col lg:flex-row justify-between w-full",
            }
        }
    }
}
