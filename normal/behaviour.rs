//! The site's browser behaviour: theme cycling and the copy-email button.
//!
//! Both are pure DOM work, so they live in one inline script instead of a
//! server round-trip per interaction.

use core::fmt::Write as _;

use libs::behaviour::COPY_EMAIL;

use crate::theme::{
    COOKIE,
    THEMES,
};

/// The inline script the document loads at the end of `<body>`.
#[must_use]
pub fn page_behaviour() -> String {
    format!(
        r#"
(() => {{
	const themes = {themes};
	const root = document.documentElement;

	const toggle = document.getElementById("theme-toggle");
	if (toggle) {{
		toggle.addEventListener("click", () => {{
			const current = root.dataset.theme || themes[0];
			const next = themes[(themes.indexOf(current) + 1) % themes.length];
			root.dataset.theme = next;
			document.cookie = "{cookie}=" + next + "; path=/; max-age=31536000; samesite=lax";
			toggle.blur();
		}});
	}}
{COPY_EMAIL}
}})();
"#,
        themes = theme_list(),
        cookie = COOKIE,
    )
}

/// The cycle order as a JavaScript array literal.
fn theme_list() -> String {
    let mut list = String::from("[");
    for (index, theme) in THEMES.iter().enumerate() {
        if index > 0 {
            list.push_str(", ");
        }
        write!(list, "{:?}", theme.as_str()).expect("writing to a String cannot fail");
    }
    list.push(']');
    list
}
