//! Routes and the editor frame.

pub mod project;

use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use topcoat::{
    Result,
    context::Cx,
    dev,
    font::{
        self,
        Font,
        fontsource::fontsource_font,
    },
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
        request,
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
    components::{
        file_info,
        footer,
        navbar,
    },
    home::home_page,
    keys,
};

/// The site font.
pub const FONT: Font = fontsource_font!(IOSEVKA, weight: [400]);

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

/// The editor frame wrapping every page.
#[layout]
async fn shell(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let (filename, filetype) = file_info(request::uri(cx).path());

    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"PlumJam — nerd"</title>
                dev::script()
                font::link(font: FONT)
                <link rel="icon" href="/assets/favicon.ico">
                <link rel="stylesheet" href="/tailwind.css">
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css">
                <script src="https://kit.fontawesome.com/6972f6e365.js" crossorigin="anonymous" fetchpriority="high"></script>
            </head>
            <body>
                <div tabindex="0" autofocus=(true) style="outline: none;">
                    <div class="min-h-screen flex flex-col min-w-full">
                        navbar()
                        <main class="flex flex-col grow max-w-6xl w-full mx-auto border-x-1 border-t-1 border-white/20 bg-[#0f1116]">
                            error_boundary(
                                fallback: |error| {
                                    if error.downcast_ref::<NotFoundError>().is_none() {
                                        return Err(error);
                                    }
                                    Ok(view! {
                                        (StatusCode::NOT_FOUND)
                                        <h1>"Page not found"</h1>
                                        <p>"404"</p>
                                    })
                                },
                                (slot)
                            )
                        </main>
                        footer(filename: filename, filetype: filetype)
                    </div>
                </div>
                <script>(Unescaped::new_unchecked(keys::behaviour()))</script>
            </body>
        </html>
    })
}

/// The home page.
#[page]
pub async fn home() -> Result<impl View> {
    Ok(view! { home_page() })
}
