//! Contact details.

use libs::data::ME;
use topcoat::{
    Result,
    icon::icon,
    router::page,
    view::{
        View,
        attributes,
        view,
    },
};

use crate::{
    components::copy_email_button,
    icons::{
        lucide,
        simple_icons,
    },
};

// Tailwind's preflight sets `svg { display: block }`; these icons sit inline
// with their text, so they are marked `inline-block`.
const ICON: &str = "w-4 mr-2 inline-block";

/// Lists the ways to get in touch.
#[page]
pub async fn page() -> Result<impl View> {
    let mailto = format!("mailto:{}", ME.email);
    let link_class = "underline ml-2 opacity-70 hover:text-purple-light hover:opacity-100 \
                      transition-all duration-100";

    Ok(view! {
        <div class="h-full w-full py-6 gap-2 flex flex-col px-4 lg:px-0">
            <p class="mb-2">
                "Matrix is my preferred way of communicating but feel free to reach out via Email or Xitter too!"
            </p>
            <p class="mb-2">
                "Happy to discuss projects, work and anything else. My CV is available upon request."
            </p>
            <p>
                icon(data: lucide::MAIL, label: "Email", attrs: attributes! { class=(ICON) })
                "Email: "
                <a class=(link_class) href=(mailto)>(ME.email)</a>
                copy_email_button()
            </p>
            <p>
                icon(data: simple_icons::MATRIX, label: "Matrix", attrs: attributes! { class=(ICON) })
                "Matrix: "
                <a class=(link_class) href="https://matrix.to/#/@plumjam:plumj.am" target="_blank" rel="noopener noreferrer">"@plumjam:plumj.am"</a>
            </p>
            <p>
                icon(data: simple_icons::X, label: "Xitter", attrs: attributes! { class=(ICON) })
                "Xitter: "
                <a class=(link_class) href="https://x.com/plumj_am" target="_blank" rel="noopener noreferrer">"@plumj_am"</a>
            </p>
            <p class="mt-8 mb-2">"Other links:"</p>
            <p>
                icon(data: simple_icons::GITHUB, label: "GitHub", attrs: attributes! { class=(ICON) })
                "GitHub: "
                <a class=(link_class) href="https://github.com/plumj-am" target="_blank" rel="noopener noreferrer">"plumj-am"</a>
            </p>
            <p>
                icon(data: simple_icons::FORGEJO, label: "Forgejo", attrs: attributes! { class=(ICON) })
                "Forgejo: "
                <a class=(link_class) href="https://git.plumj.am/plumjam" target="_blank" rel="noopener noreferrer">"plumjam"</a>
            </p>
            <p>
                icon(data: simple_icons::NPM, label: "NPM", attrs: attributes! { class=(ICON) })
                "NPM: "
                <a class=(link_class) href="https://www.npmjs.com/~jamesukiyo" target="_blank" rel="noopener noreferrer">"jamesukiyo"</a>
            </p>
            <p>
                icon(data: simple_icons::RUST, label: "Crates.io", attrs: attributes! { class=(ICON) })
                "Crates: "
                <a class=(link_class) href="https://crates.io/users/plumj-am" target="_blank" rel="noopener noreferrer">"plumj-am"</a>
            </p>
        </div>
    })
}
