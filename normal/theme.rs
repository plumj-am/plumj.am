use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Theme {
    pub fn to_string(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::Auto => "auto",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }

    pub fn cycle(self) -> Self {
        match self {
            Theme::Auto => Theme::Light,
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Auto,
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Theme::Light => "fas fa-sun",
            Theme::Dark => "fas fa-moon",
            Theme::Auto => "fas fa-circle-half-stroke",
        }
    }
}

/// Global theme context provider.
#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let mut theme = use_signal(|| {
        // Try to load from localStorage, default to Auto.
        Theme::Auto
    });

    use_effect(move || {
        let theme_val = theme();

        // Apply theme to document.
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", theme_val.to_string());

                // Store in localStorage.
                if let Some(storage) =
                    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
                {
                    let _ = storage.set_item("theme", theme_val.to_string());
                }
            }
        }
    });

    // Load theme from localStorage on mount.
    use_effect(move || {
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Ok(Some(stored_theme)) = storage.get_item("theme") {
                theme.set(Theme::from_string(&stored_theme));
            }
        }
    });

    use_context_provider(|| theme);

    rsx! {
        {children}
    }
}

/// Hook to access theme state.
pub fn use_theme() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme = use_theme();
    let current_theme = theme();

    rsx! {
        button {
            class: "flex items-center justify-center w-auto h-6 border-fg bg-transparent hover:bg-[var(--color-hover)] transition-all duration-200 px-1",
            onclick: move |_| {
                theme.set(current_theme.cycle());
            },
            title: format!("Current theme: {} (click to cycle)", match current_theme {
                Theme::Light => "Light",
                Theme::Dark => "Dark",
                Theme::Auto => "Auto (system)",
            }),
            p { class: "text-xs mr-1",
                "Theme:",
                {match current_theme {
                    Theme::Light => "Light",
                    Theme::Dark => "\u{00A0}Dark",
                    Theme::Auto => "\u{00A0}Auto",
                }}
            },
            i {
                class: format!("{} text-fg text-sm", current_theme.icon())
            }
        }
    }
}
