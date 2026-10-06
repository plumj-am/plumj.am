//! Routes and the document layout.

pub mod blog;
pub mod contact;
pub mod projects;

use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use libs::data::{
    ME,
    PROJECTS,
};
use topcoat::{
    Result,
    context::Cx,
    cookie::RouterBuilderCookieExt as _,
    dev,
    font::{
        self,
        Font,
        fontsource::fontsource_font,
    },
    icon::icon,
    router::{
        Router,
        RouterBuilderDirectoryExt as _,
        RouterBuilderDiscoverExt as _,
        Slot,
        StatusCode,
        content::Css,
        error::NotFoundError,
        layout,
        module_router,
        not_found,
        page,
        route,
    },
    view::{
        Unescaped,
        View,
        error_boundary,
        view,
    },
};

use crate::{
    behaviour,
    components::{
        LOGO,
        copy_email_button,
        footer,
        navbar,
        project_card,
    },
    icons::simple_icons,
    theme,
};

/// The site font.
pub const FONT: Font = fontsource_font!(JETBRAINS_MONO, weight: [400]);

/// `GET /tailwind.css` - the stylesheet rendered by the build script.
#[route(GET "/tailwind.css")]
async fn stylesheet() -> Result<Css<&'static [u8]>> {
    Ok(Css(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/tailwind.css"
    ))))
}

/// Builds the site router.
#[must_use]
pub fn router() -> Router {
    module_router!()
        .discover()
        .cookies()
        .serve_dir("/assets/{*rest}", assets_dir())
        .build()
}

/// The directory holding the static files served below `/assets`.
///
/// A deployment points `PLUMJAM_ASSETS_DIR` at the installed copy; during
/// development the crate's own `assets` directory is used.
fn assets_dir() -> PathBuf {
    env::var_os("PLUMJAM_ASSETS_DIR").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("assets"),
        PathBuf::from,
    )
}

// Serves every otherwise unmatched URL through the layouts.
not_found!();

/// The document wrapping every page.
#[layout]
async fn shell(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html data-theme=(theme::current(cx).as_str())>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"PlumJam"</title>
                dev::script()
                font::link(font: FONT)
                <link rel="icon" href="/assets/favicon.ico">
                <link rel="stylesheet" href="/tailwind.css">
            </head>
            <body class="min-h-screen flex flex-col min-w-full">
                navbar()
                <main class="flex flex-col grow max-w-5xl w-full mx-auto">
                    error_boundary(
                        fallback: |error| {
                            if error.downcast_ref::<NotFoundError>().is_none() {
                                return Err(error);
                            }
                            Ok(view! {
                                (StatusCode::NOT_FOUND)
                                <div class="w-full py-16 px-4 lg:px-0">
                                    <h1 class="text-3xl font-bold mb-3">"Page not found"</h1>
                                    <p class="opacity-70">"404 — this page does not exist."</p>
                                </div>
                            })
                        },
                        (slot)
                    )
                </main>
                footer()
                <script>(Unescaped::new_unchecked(behaviour::page_behaviour()))</script>
            </body>
        </html>
    })
}

/// The home page: a profile column next to a project teaser.
#[page]
pub async fn home() -> Result<impl View> {
    let learning = ME.learning_langs.join(", ");
    let mailto = format!("mailto:{}", ME.email);

    Ok(view! {
        <div class="flex w-full flex-col lg:flex-row gap-8 py-6 items-center lg:items-start">
            <div class="flex flex-col gap-6 lg:w-1/2">
                <div class="text-center lg:text-left">
                    <img class="w-32 mx-auto lg:mx-0 mt-2 mb-4 hover:scale-120 transition-all duration-300" src=(LOGO) alt="PlumJam">
                    <h1 class="text-3xl font-bold mb-2 text-purple-light">(ME.name)</h1>
                    <p class="text-fg opacity-70 mb-4">
                        <a class="hover:opacity-100 transition-opacity" href=(mailto)>(ME.email)</a>
                        copy_email_button()
                    </p>
                    <hr class="opacity-10">
                    <div class="px-2 md:px-0 max-w-prose">
                        <p class="text-fg mt-4">"Self-taught software developer since 2024."</p>
                        <p class="text-fg mt-4">"Trying to shift my focus away from the web dev world..."</p>
                        <p class="text-fg mt-4">"Currently enjoying and learning: "(learning)"."</p>
                        <p class="text-fg mt-4 mb-4">"If you're looking for help with a project or a new teammate, shoot me an email! :]"</p>
                    </div>
                    <hr class="opacity-10">
                </div>

                <div class="flex flex-wrap gap-8 sm:gap-32 px-2 md:px-0 mx-auto lg:mx-0">
                    <div>
                        <h3 class="font-semibold mb-2 text-fg">"Languages"</h3>
                        <ul class="text-sm text-fg opacity-80 space-y-1">
                            for lang in ME.langs {
                                <li>"• "(lang)</li>
                            }
                        </ul>
                    </div>
                    <div>
                        <h3 class="font-semibold mb-2 text-fg">"Frameworks"</h3>
                        <ul class="text-sm text-fg opacity-80 space-y-1">
                            for framework in ME.frameworks {
                                <li>"• "(framework)</li>
                            }
                        </ul>
                    </div>
                </div>
                <hr class="opacity-10">
                <div class="flex gap-4 justify-center lg:justify-start">
                    for social in ME.socials {
                        <a class="group flex items-center text-fg hover:text-purple-light transition-all duration-100 overflow-hidden p-1" href=(social.url) target="_blank" rel="noopener noreferrer">
                            match social.name {
                                "GitHub" => icon(data: simple_icons::GITHUB, label: "GitHub"),
                                "Forgejo" => icon(data: simple_icons::FORGEJO, label: "Forgejo"),
                                "Xitter" => icon(data: simple_icons::X, label: "X"),
                                "Matrix" => icon(data: simple_icons::MATRIX, label: "Matrix"),
                                _ => "",
                            }
                            <span class="ml-2 hidden sm:flex transition-all duration-100 whitespace-nowrap">(social.name)</span>
                        </a>
                    }
                </div>
            </div>

            <div class="flex flex-col gap-6 lg:w-1/2 px-4 lg:px-0">
                <h2 class="text-2xl font-bold text-fg">"Projects (5/"(PROJECTS.len())")"</h2>
                <div class="grid sm:grid-cols-2 lg:grid-cols-1 gap-4">
                    for project in PROJECTS.iter().take(5) {
                        project_card(project: project)
                    }
                </div>
            </div>
        </div>
    })
}
