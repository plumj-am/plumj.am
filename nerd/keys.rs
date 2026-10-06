//! Vim-style scrolling keys and the browser behaviour behind them.

use libs::behaviour::COPY_EMAIL;

/// A scroll target for a keypress.
#[derive(Clone, Copy)]
pub enum ScrollDir {
    Top,
    LineUp,
    HalfUp,
    HalfDown,
    LineDown,
    Bottom,
}

impl ScrollDir {
    /// The name of the browser function that performs the scroll.
    #[must_use]
    pub const fn js(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::LineUp => "lineUp",
            Self::HalfUp => "halfUp",
            Self::HalfDown => "halfDown",
            Self::LineDown => "lineDown",
            Self::Bottom => "bottom",
        }
    }
}

/// One keybinding, shown in the navbar and handled in the browser.
pub struct Keymap {
    pub key:    &'static str,
    pub action: ScrollDir,
    pub desc:   &'static str,
}

pub const KEYMAPS: &[Keymap] = &[
    Keymap {
        key:    "gg",
        action: ScrollDir::Top,
        desc:   "top",
    },
    Keymap {
        key:    "G",
        action: ScrollDir::Bottom,
        desc:   "bottom",
    },
    Keymap {
        key:    "u",
        action: ScrollDir::HalfUp,
        desc:   "½ up",
    },
    Keymap {
        key:    "d",
        action: ScrollDir::HalfDown,
        desc:   "½ down",
    },
    Keymap {
        key:    "k",
        action: ScrollDir::LineUp,
        desc:   "line up",
    },
    Keymap {
        key:    "j",
        action: ScrollDir::LineDown,
        desc:   "line down",
    },
];

/// The site's browser behaviour: vim-style scrolling, scroll snapping to the
/// 24px line grid, and the copy-email button.
///
/// All of it is pure DOM work, so it lives in one self-contained script.
#[must_use]
pub fn behaviour() -> String {
    use core::fmt::Write as _;

    let mut bindings = String::new();
    for keymap in KEYMAPS {
        write!(bindings, "{:?}: {:?},", keymap.key, keymap.action.js())
            .expect("writing to a String cannot fail");
    }

    format!(
        r#"
(() => {{
	const LINE = 24;
	const scroll = {{
		top: () => window.scrollTo(0, 0),
		bottom: () => window.scrollTo(0, 9999.9),
		lineUp: () => window.scrollTo(0, window.scrollY - LINE),
		lineDown: () => window.scrollTo(0, window.scrollY + LINE),
		halfUp: () => window.scrollTo(0, window.scrollY - window.innerHeight),
		halfDown: () => window.scrollTo(0, window.scrollY + window.innerHeight),
	}};
	const keys = {{ {bindings} }};

	// "gg" scrolls to the top; any other key resets the pending first "g".
	let g = false;
	window.addEventListener("keydown", (event) => {{
		if (event.key === "g") {{
			if (g) scroll.top();
			g = !g;
			return;
		}}
		const action = keys[event.key];
		if (action) scroll[action]();
		g = false;
	}});

	// Snap the viewport to the line grid after scrolling settles.
	let timeout;
	window.addEventListener("scroll", () => {{
		clearTimeout(timeout);
		timeout = setTimeout(() => {{
			const y = window.pageYOffset;
			const target = Math.round(y / LINE) * LINE;
			if (Math.abs(y - target) > 1) window.scrollTo({{ top: target, behavior: "auto" }});
		}}, 100);
	}}, {{ passive: true }});
{COPY_EMAIL}
}})();
"#,
    )
}
