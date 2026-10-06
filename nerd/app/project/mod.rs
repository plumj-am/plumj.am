//! A single project "file".

use libs::data::{
    PROJECTS,
    ProjectInfo,
};
use topcoat::{
    Result,
    context::Cx,
    router::{
        href,
        page,
        path_param,
    },
    runtime::link,
    view::{
        View,
        attributes,
        component,
        view,
    },
};

use crate::lines::{
    hi_cursor,
    hi_line,
    line,
    line_numbers,
};

path_param!(pub name);

/// Renders the project identified by the URL segment.
#[page("./{name}")]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let name: &str = path_param::<Name>(cx);

    Ok(view! {
        <div class="flex w-full">
            hi_line()
            line_numbers(max_lines: 70)
            <div class="flex flex-col w-full">
                hi_cursor()
                match PROJECTS.iter().find(|project| project.clean_name() == name) {
                    Some(project) => project_detail(project: project),
                    None => project_missing(name: name.to_owned()),
                }
            </div>
        </div>
    })
}

/// The details of one project.
#[component]
async fn project_detail(project: &'static ProjectInfo) -> Result<impl View> {
    Ok(view! {
        <h1>"Name: "(project.name)</h1>
        if project.long_desc.is_some() {
            line()
            <p>"Description: "(project.long_desc.unwrap_or(""))</p>
        }
        line()
        <p>"Type: "(project.project_type.as_str())</p>
        line()
        <span>
            <i class="fa-brands fa-github mr-1"></i>
            "GitHub: "
            match project.github_url() {
                Some(url) => {
                    <a class="text-white/90 hover:underline hover:text-[#A66AA2]" href=(url.as_str()) target="_blank" rel="noopener noreferrer">
                        <span>(url.replace("https://", " "))</span>
                    </a>
                },
                None => <span class="opacity-40">"Currently private."</span>,
            }
        </span>
        <span>
            <i class="fa-brands fa-git-alt mr-1"></i>
            "Forgejo: "
            match project.forgejo_url() {
                Some(url) => {
                    <a class="text-white/90 hover:underline hover:text-[#A66AA2]" href=(url.as_str()) target="_blank" rel="noopener noreferrer">
                        <span>(url.replace("https://", " "))</span>
                    </a>
                },
                None => <span class="opacity-40">"Currently private."</span>,
            }
        </span>
        if let Some(url) = project.npm_url {
            <span>
                <i class="fa-brands fa-npm mr-1"></i>
                "NPM: "
                <a class="text-white/90 hover:underline hover:text-[#A66AA2]" href=(url) target="_blank" rel="noopener noreferrer">
                    <span>(url.replace("https://www.", " "))</span>
                </a>
            </span>
        }
        if let Some(url) = project.crate_url {
            <span>
                <i class="fa-brands fa-rust mr-1"></i>
                "Crate: "
                <a class="text-white/90 hover:underline hover:text-[#A66AA2]" href=(url) target="_blank" rel="noopener noreferrer">
                    <span>(url.replace("https://", " "))</span>
                </a>
            </span>
        }
        if let Some(url) = project.site_url {
            <span>
                <i class="fa fa-globe mr-1"></i>
                "Site: "
                <a class="text-white/90 hover:underline hover:text-[#A66AA2]" href=(url) target="_blank" rel="noopener noreferrer">
                    <span>
                        (url.replace("https://", " "))
                        if project.name == "plumj.am" {
                            <span class="opacity-40">" (You're already here :)"</span>
                        }
                    </span>
                </a>
            </span>
        }
        line()
        <h2>"Technologies used:"</h2>
        <ul>
            for technology in project.tech_used {
                <li>"• "(technology)</li>
            }
        </ul>
        line()
        if project.media.is_some() {
            <h2>"Media:"</h2>
        }
    })
}

/// The view shown when no project matches the URL segment.
#[component]
async fn project_missing(name: String) -> Result<impl View> {
    Ok(view! {
        <h1>"Project not found"</h1>
        line()
        <p>"Project '"(name)"' not found"</p>
        link(
            href: href!(crate::app::home),
            attrs: attributes! { class="text-white decoration-none hover:cursor-pointer hover:opacity-80 hover:underline" },
            "← Back to Home"
        )
    })
}
