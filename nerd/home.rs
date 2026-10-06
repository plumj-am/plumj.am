//! The home "file": profile on the left, projects on the right.

use libs::data::{
    ME,
    PROJECTS,
};
use topcoat::{
    Result,
    router::href,
    view::{
        View,
        component,
        view,
    },
};

use crate::{
    app::project,
    components::copy_email_button,
    lines::{
        LineType,
        hi_cursor,
        hi_line,
        line,
        line_numbers,
    },
};

/// The home page.
#[component]
pub async fn home_page() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full">
            hi_line()
            line_numbers(max_lines: 70)
            <div class="flex flex-col lg:flex-row justify-between w-full">
                profile()
                projects()
            </div>
        </div>
    })
}

/// The left column: the profile block.
#[component]
pub async fn profile() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col">
            hi_cursor()
            line(type_of: LineType::H1, text: "Profile".to_owned(), classes: "w-fit relative")
            line()
            <div class="flex flex-col items-start">
                <img class="w-36 h-36 object-fit rounded-sm" src=(ME.image_url) alt=(ME.name)>
            </div>
            line()
            <div class="line-content">
                // `\u{00A0}` is non-breaking space character for alignment
                <p>"Email: \u{00A0}\u{00A0} "(ME.email)</p>
                copy_email_button()
            </div>
            line(type_of: LineType::P, text: format!("Location: {}", ME.location), classes: "")
            line()
            <div class="grid grid-cols-2 gap-x-8">
                <div class="flex flex-col">
                    line(type_of: LineType::H2, text: "Languages:".to_owned(), classes: "mb-[-1px]")
                    for lang in ME.langs {
                        line(type_of: LineType::P, text: format!("- {lang}"), classes: "")
                    }
                </div>
                <div class="flex flex-col">
                    line(type_of: LineType::H2, text: "Scripting:".to_owned(), classes: "")
                    for script in ME.scripting {
                        line(type_of: LineType::P, text: format!("- {script}"), classes: "")
                    }
                </div>
            </div>
            line()
            <div class="grid grid-cols-2 gap-x-8">
                <div class="flex flex-col">
                    line(type_of: LineType::H2, text: "Frameworks:".to_owned(), classes: "mb-[-1px]")
                    for framework in ME.frameworks {
                        line(type_of: LineType::P, text: format!("- {framework}"), classes: "")
                    }
                </div>
                <div class="flex flex-col">
                    line(type_of: LineType::H2, text: "Tools:".to_owned(), classes: "mb-[-1px]")
                    for tool in ME.tools {
                        line(type_of: LineType::P, text: format!("- {tool}"), classes: "")
                    }
                </div>
            </div>
            line()
            <div class="flex flex-col">
                <img class="max-w-md" src="https://ghchart.rshah.org/592D59/plumj-am" alt="GitHub Contribution Chart">
            </div>
            line()
            <div class="line-content flex flex-row gap-x-6 mt-1">
                for social in ME.socials {
                    <a class="flex items-center text-white hover:text-[#A66AA2] hover:scale-110" href=(social.url) target="_blank" rel="noopener noreferrer">
                        <i class=(format!("{} text-lg h-fit w-fit", social.icon))></i>
                        <p class="ml-2">(social.name)</p>
                    </a>
                }
            </div>
            line()
        </div>
    })
}

/// The right column: the project list.
#[component]
pub async fn projects() -> Result<impl View> {
    Ok(view! {
        <div class="text-xl mr-1">
            line(type_of: LineType::H1, text: format!("Projects ({})", PROJECTS.len()), classes: "w-fit")
            line()
            <div class="text-left text-white flex flex-col gap-4">
                for project_info in PROJECTS {
                    <div class="group text-white w-xl border-1 border-white rounded-md pt-5 px-4 pb-3 hover:bg-[#3C2240] hover:scale-110 relative">
                        <a
                            class="block hover:cursor-pointer"
                            href=(href!(project::page, project::Name(project_info.clean_name())))
                        >
                            <div class="flex flex-row justify-between border-b-1 border-white/20">
                                <span class="pb-2">
                                    (project_info.name)
                                    " "
                                    <span class="opacity-0 group-hover:opacity-100 text-[#F2EEEB]/80">"-- "(project_info.short_desc)</span>
                                </span>
                                <span class="text-white/60 text-lg">"["(project_info.project_type.as_str())"]"</span>
                            </div>
                            <div class="flex flex-row justify-between items-end">
                                <span class="pt-2 text-sm text-white/60">(project_info.tech_used_str())</span>
                            </div>
                        </a>
                        <div class="absolute bottom-3 right-4 flex gap-2 items-center">
                            match project_info.github_url() {
                                Some(url) => {
                                    <a class="opacity-80 hover:opacity-100" href=(url) target="_blank" rel="noopener noreferrer">
                                        <i class="fa-brands fa-github text-white text-lg"></i>
                                    </a>
                                },
                                None => "",
                            }
                            match project_info.forgejo_url() {
                                Some(url) => {
                                    <a class="opacity-80 hover:opacity-100" href=(url) target="_blank" rel="noopener noreferrer">
                                        <i class="fa-brands fa-git-alt text-white text-lg"></i>
                                    </a>
                                },
                                None => "",
                            }
                            if let Some(url) = project_info.npm_url {
                                <a class="opacity-80 hover:opacity-100" href=(url) target="_blank" rel="noopener noreferrer">
                                    <i class="fa-brands fa-npm text-white text-lg"></i>
                                </a>
                            }
                            if let Some(url) = project_info.crate_url {
                                <a class="opacity-80 hover:opacity-100" href=(url) target="_blank" rel="noopener noreferrer">
                                    <i class="fa-brands fa-rust text-white text-lg"></i>
                                </a>
                            }
                            if let Some(url) = project_info.site_url {
                                <a class="opacity-80 hover:opacity-100" href=(url) target="_blank" rel="noopener noreferrer">
                                    <i class="fa fa-globe text-white text-lg"></i>
                                </a>
                            }
                        </div>
                    </div>
                }
            </div>
        </div>
    })
}
