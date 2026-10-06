//! The editor chrome: the fixed navbar, the vim-style statusline, and the
//! copy-email button.

use libs::data::{
    ME,
    PROJECTS,
    VERSION,
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

use crate::keys::KEYMAPS;

/// The filename and filetype the statusline shows for a request path.
#[must_use]
pub fn file_info(path: &str) -> (String, String) {
    let Some(name) = path
        .strip_prefix("/project/")
        .and_then(|rest| rest.split('/').next())
    else {
        return ("portfolio/main.rs".to_owned(), "rust".to_owned());
    };

    let tech = PROJECTS
        .iter()
        .find(|project| project.clean_name() == name)
        .map_or("rust", |project| project.main_tech_used())
        .to_lowercase();

    let extension = match tech.as_str() {
        "rust" => "rs",
        "c++" => "cpp",
        "sveltekit" => "svelte",
        "vimscript" => "vim",
        "nushell" => "nu",
        _ => &tech,
    };

    // Strip file extensions from the project name and use underscores.
    let clean_name = name
        .replace(['.', '-'], "_")
        .replace("_nvim", "")
        .replace("_vim", "")
        .replace("_rs", "")
        .replace("_github_io", "");

    (format!("portfolio/{clean_name}.{extension}"), tech)
}

/// The top bar: the home shortcut, the keymap hints and the social links.
#[component]
pub async fn navbar() -> Result<impl View> {
    Ok(view! {
        <div class="h-12 bg-[#0f1116]"></div>
        <div class="fixed top-0 z-99 left-0 right-0 bg-[#16161e] h-12 w-full flex flex-row items-center max-w-6xl px-3 mx-auto border-x-1 border-t-1 border-x-white/20 border-t-white/20">
            <div class="flex items-center">
                <a class="flex items-center hover:text-[#A66AA2] hover:scale-150" href=(href!(crate::app::home))>
                    <i class="fa-solid fa-house"></i>
                </a>
            </div>
            <div class="flex-1 flex justify-center">
                <div class="flex items-center gap-x-3 text-sm">
                    <i class="devicon-vim-plain text-2xl"></i>
                    <div class="flex items-center gap-x-4">
                        for keymap in KEYMAPS {
                            <div class="flex items-center gap-x-2">
                                <kbd class="bg-gray-700 px-1 rounded text-md">(keymap.key)</kbd>
                                <span class="text-sm! font-mono">(keymap.desc)</span>
                            </div>
                        }
                    </div>
                </div>
            </div>
            <div class="flex flex-row gap-x-4 items-center">
                <p class="text-sm text-white/90">"v"(VERSION)</p>
                for social in ME.socials {
                    <a class="text-white hover:text-[#A66AA2] hover:scale-150" href=(social.url) target="_blank" rel="noopener noreferrer">
                        <i class=(social.icon)></i>
                    </a>
                }
            </div>
        </div>
    })
}

/// The vim-style statusline at the bottom of the page.
#[component]
pub async fn footer(filename: String, filetype: String) -> Result<impl View> {
    let (icon, display) = match filetype.as_str() {
        "lua" => ("lua", "lua"),
        "bash" => ("bash", "bash"),
        "c++" => ("cplusplus", "cpp"),
        "sveltekit" | "svelte" => ("svelte", "svelte"),
        "vimscript" => ("vim", "vim"),
        "nushell" | "nu" => ("nu", "nu"),
        _ => ("rust", "rust"),
    };

    Ok(view! {
        <div class="fixed max-w-6xl bottom-0 left-0 right-0 bg-[#16161e] text-white text-md flex z-50 items-center mx-auto">
            // mode
            <div class="bg-[#A66AA2] text-[#F2EEEB] px-3">"NORMAL"</div>
            // branch
            <div class="bg-[#592D59] text-[#F2EEEB] px-3">
                <span class="flex items-center">
                    <i class="devicon-git-plain mr-1"></i>
                    "master"
                </span>
            </div>
            // file name
            <div class="bg-[#3C2240] text-[#F2EEEB] px-3">(filename)</div>
            // spacer
            <div class="flex-grow"></div>
            // file type and position + copyright
            <div class="flex items-center">
                <div class="text-white/30 text-xs mr-2">"Copyright © 2025-present - PlumJam <git@plumj.am>"</div>
                <div class="bg-[#3C2240] text-[#F2EEEB] px-3 flex items-center">
                    if icon.starts_with("nu") {
                        <span class="flex items-center">
                            <i class="fa-solid fa-terminal mr-1 text-sm"></i>
                            "nu"
                        </span>
                    } else {
                        <span class="flex items-center">
                            <i class=(format!("devicon-{icon}-plain mr-1"))></i>
                            (display)
                        </span>
                    }
                </div>
                <div class="bg-[#592D59] text-[#F2EEEB] px-3">"42:12"</div>
                <div class="bg-[#A66AA2] text-[#F2EEEB] px-3">"100%"</div>
            </div>
        </div>
    })
}

/// A button that copies the contact email; the toast is in [`crate::keys`].
#[component]
pub async fn copy_email_button() -> Result<impl View> {
    Ok(view! {
        <button
            id="copy-email"
            data-email=(ME.email)
            class="hover:cursor-pointer text-gray-500 hover:text-gray-700 transition-colors ml-2"
            title="Copy email address"
        >
            <i class="fa fa-copy text-sm opacity-80 hover:opacity-100"></i>
        </button>
    })
}
