//! Cookie-backed colour themes.
//!
//! The theme is rendered on the server from the cookie, so there is no flash
//! before paint. Clicking cycles it in the browser; see [`crate::behaviour`].

use topcoat::{
    Result,
    context::Cx,
    cookie::{
        Cookies as _,
        cookies,
    },
    icon::icon,
    view::{
        View,
        component,
        view,
    },
};

use crate::icons::lucide;

/// The cookie the chosen theme is stored in.
pub const COOKIE: &str = "theme";

/// The available colour themes, in click-cycle order.
pub const THEMES: &[Theme] = &[Theme::Auto, Theme::Light, Theme::Dark];

/// A colour theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

impl Theme {
    /// The value stored in the cookie and rendered into `data-theme`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Parses a cookie value, defaulting to [`Theme::Auto`] for anything else.
impl From<&str> for Theme {
    fn from(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::Auto,
        }
    }
}

/// Reads the visitor's chosen theme.
#[must_use]
pub fn current(cx: &Cx) -> Theme {
    cookies(cx)
        .get(COOKIE)
        .map_or(Theme::Auto, |cookie| Theme::from(cookie.value()))
}

/// Cycles the theme when clicked.
///
/// All three variants are rendered; `input.css` shows only the one matching
/// the `data-theme` attribute the script sets on the document element.
#[component]
pub async fn switcher() -> Result<impl View> {
    Ok(view! {
        <button
            id="theme-toggle"
            class="flex items-center justify-center w-auto h-6 hover:bg-[var(--color-hover)] hover:cursor-pointer transition-colors duration-100 px-1"
            title="Click to cycle the theme"
        >
            <span class="text-xs mr-1">"Theme:"</span>
            <span class="text-xs theme-option theme-light">"Light"</span>
            <span class="text-xs theme-option theme-dark">"Dark"</span>
            <span class="text-xs theme-option theme-auto">"Auto"</span>
            <span class="theme-option theme-light">icon(data: lucide::SUN, label: "Light theme")</span>
            <span class="theme-option theme-dark">icon(data: lucide::MOON, label: "Dark theme")</span>
            <span class="theme-option theme-auto">icon(data: lucide::CONTRAST, label: "System theme")</span>
        </button>
    })
}
