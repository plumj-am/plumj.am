//! Views shared across pages.

use libs::data::{
    ME,
    ProjectInfo,
    VERSION,
};
use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    router::{
        href,
        request,
    },
    view::{
        StaticClass,
        View,
        class,
        component,
        view,
    },
};

use crate::{
    icons::{
        lucide,
        simple_icons,
    },
    theme::switcher,
};

/// The site logo, served from the crate's `assets` directory.
pub const LOGO: &str = "/assets/plumjam-nobg.png";

const NAV_ITEM: StaticClass = class!(
    "group flex items-center justify-center w-full h-full border-fg hover:bg-[var(--color-hover)] \
     transition-scale duration-100 hover:scale-120 hover:shadow-[inset_0_0_0_1px_var(--color-fg)] \
     hover:border-0 border-r-1 hover:z-10"
);

const NAV_ACTIVE: StaticClass = class!(
    "text-[var(--color-fg)] bg-[var(--color-active)] hover:bg-[var(--color-active)] \
     text-base-light border-fg border-r-1"
);

/// The top navigation bar with the section links and the theme switcher.
#[component]
pub async fn navbar(cx: &Cx) -> Result<impl View> {
    let path = request::uri(cx).path().to_owned();
    let on_home = path == "/";
    let on_projects = path.starts_with("/projects");
    let on_blog = path.starts_with("/blog");
    let on_contact = path.starts_with("/contact");

    Ok(view! {
        <div class="h-16 border-l-1 border-b-1 border-fg w-full max-w-5xl mx-auto relative">
            <div class="grid grid-cols-4 w-full h-full text-md sm:text-xl md:text-2xl tracking">
                <a href=(href!(crate::app::home)) class=(class!(NAV_ITEM, NAV_ACTIVE if on_home))>
                    <p class=(class!("transition-scale duration-100", "sm:scale-150" if on_home else "sm:group-hover:scale-150"))>"HOME"</p>
                </a>
                <a href=(href!(crate::app::projects::page)) class=(class!(NAV_ITEM, NAV_ACTIVE if on_projects))>
                    <p class=(class!("transition-scale duration-100", "sm:scale-150" if on_projects else "sm:group-hover:scale-150"))>"PROJECTS"</p>
                </a>
                <a href=(href!(crate::app::blog::page)) class=(class!(NAV_ITEM, NAV_ACTIVE if on_blog))>
                    <p class=(class!("transition-scale duration-100", "sm:scale-150" if on_blog else "sm:group-hover:scale-150"))>"BLOG"</p>
                </a>
                <a href=(href!(crate::app::contact::page)) class=(class!(NAV_ITEM, NAV_ACTIVE if on_contact))>
                    <p class=(class!("transition-scale duration-100", "sm:scale-150" if on_contact else "sm:group-hover:scale-150"))>"CONTACT"</p>
                </a>
            </div>
            <div class="absolute bottom-[-25] right-0">
                switcher()
            </div>
        </div>
    })
}

/// The page footer.
#[component]
pub async fn footer() -> Result<impl View> {
    Ok(view! {
        <div class="relative group flex items-center justify-center h-18 sm:h-8 border-t-1 border-x-1 border-fg w-full max-w-5xl mx-auto">
            <p class="sm:text-xs flex flex-col sm:flex-row items-center">
                <span class="md:group-hover:translate-x-[-20px] transition-translate duration-300">"Copyright © 2025-present"</span>
                <img class="h-4 px-2 md:group-hover:scale-500 md:group-hover:translate-y-[-40px] transition-translate-y duration-300 animate-bounce" src=(LOGO) alt="PlumJam">
                <span class="md:group-hover:translate-x-[20px] transition-translate duration-300">
                    <span class="text-purple-light pr-2">"PlumJam"</span>
                    "<git@plumj.am>"
                </span>
            </p>
            <p class="absolute text-xs bottom-1 sm:bottom-2 right-1">"v"(VERSION)</p>
        </div>
    })
}

/// The "find me elsewhere" block shown under a post.
#[component]
pub async fn credits() -> Result<impl View> {
    Ok(view! {
        <div class="mt-8 w-full">
            <hr class="opacity-20">
            <p class="mt-8 mb-4 flex flex-col">
                "You can find me on..."
                <p class="mt-2">
                    "Xitter: "
                    <a class="ml-2 hover:text-purple-light opacity-80 hover:opacity-100" href="https://x.com/plumj_am">"@plumj_am"</a>
                </p>
                <p>
                    "GitHub: "
                    <a class="ml-2 hover:text-purple-light opacity-80 hover:opacity-100" href="https://github.com/plumj-am">"plumj-am"</a>
                </p>
                <p>
                    "Forgejo: "
                    <a class="ml-2 hover:text-purple-light opacity-80 hover:opacity-100" href="https://git.plumj.am/plumjam">"plumjam"</a>
                </p>
                <p>
                    "Matrix: "
                    <a class="ml-2 hover:text-purple-light opacity-80 hover:opacity-100" href="https://matrix.to/#/@plumjam:plumj.am">"@plumjam:plumj.am"</a>
                </p>
            </p>
        </div>
    })
}

/// A project card with its type, description, tech, and repository links.
#[component]
pub async fn project_card(project: &'static ProjectInfo) -> Result<impl View> {
    Ok(view! {
        <div class="border border-fg p-4 hover:bg-purple-light transition-scale hover:scale-105 md:hover:scale-110 duration-100 relative hover:z-10">
            <div class="flex justify-between items-start mb-2">
                <h3 class="font-semibold text-fg">(project.name)</h3>
                <span class="text-xs px-2 py-1 bg-green-light text-fg">(project.project_type.as_str())</span>
            </div>
            <p class="text-sm text-fg opacity-80 mb-3">(project.short_desc)</p>
            <div class="flex justify-between items-center">
                <span class="text-xs text-fg opacity-60">(project.tech_used_str())</span>
                <div class="flex gap-2">
                    if let Some(url) = project.github_url() {
                        <a class="text-fg hover:opacity-70" href=(url) target="_blank" rel="noopener noreferrer">
                            icon(data: simple_icons::GITHUB, label: "GitHub")
                        </a>
                    }
                    if let Some(url) = project.forgejo_url() {
                        <a class="text-fg hover:opacity-70" href=(url) target="_blank" rel="noopener noreferrer">
                            icon(data: simple_icons::FORGEJO, label: "Forgejo")
                        </a>
                    }
                    if let Some(url) = project.npm_url {
                        <a class="text-fg hover:opacity-70" href=(url) target="_blank" rel="noopener noreferrer">
                            icon(data: simple_icons::NPM, label: "npm")
                        </a>
                    }
                    if let Some(url) = project.crate_url {
                        <a class="text-fg hover:opacity-70" href=(url) target="_blank" rel="noopener noreferrer">
                            icon(data: simple_icons::RUST, label: "crates.io")
                        </a>
                    }
                    if let Some(url) = project.site_url {
                        <a class="text-fg hover:opacity-70" href=(url) target="_blank" rel="noopener noreferrer">
                            icon(data: lucide::GLOBE, label: "Website")
                        </a>
                    }
                </div>
            </div>
        </div>
    })
}

/// A button that copies the contact email to the clipboard.
#[component]
pub async fn copy_email_button() -> Result<impl View> {
    Ok(view! {
        <button
            id="copy-email"
            data-email=(ME.email)
            class="hover:cursor-pointer text-gray-500 hover:text-gray-700 transition-colors ml-2"
            title="Copy email address"
        >
            icon(data: lucide::COPY, label: "Copy")
        </button>
    })
}
