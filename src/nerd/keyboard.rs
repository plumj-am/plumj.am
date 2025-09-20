use dioxus::prelude::*;

pub enum ScrollDir {
    Top,
    LineUp,
    HalfUp,
    HalfDown,
    LineDown,
    Bottom,
}

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

// pub enum NavOpts {
// 	Home,
// }

pub struct DoubleKeys {
    pub g: bool,
}

impl DoubleKeys {
    pub fn is_g(&self) -> bool {
        self.g
    }
}

pub fn use_keyboard_handler() -> impl FnMut(KeyboardEvent) + 'static {
    let mut double_taps = use_signal(|| DoubleKeys { g: false });

    move |evt: KeyboardEvent| {
        if let Key::Character(c) = evt.key() {
            let key_str = c.as_str();

            // handle g
            if key_str == "g" {
                if double_taps.with(DoubleKeys::is_g) {
                    scroll(&ScrollDir::Top);
                } else {
                    double_taps.with_mut(|keys| keys.g = true);
                    return;
                }
            } else {
                // handle others
                for keymap in KEYMAPS {
                    if keymap.key == key_str {
                        scroll(&keymap.action);
                        break;
                    }
                }
            }
        }
        double_taps.with_mut(|keys| keys.g = false);
    }
}

pub fn scroll(dir: &ScrollDir) {
    let w = web_sys::window().expect("window not available");
    let curr_y = w.scroll_y().unwrap_or(0.0);
    let viewport_h = w
        .inner_height()
        .expect("window inner height unavailable")
        .as_f64()
        .unwrap_or(500.0);

    match dir {
        ScrollDir::Top => {
            w.scroll_to_with_x_and_y(0.0, 0.0);
        },
        ScrollDir::LineUp => {
            // 24 px matches line height
            w.scroll_to_with_x_and_y(0.0, curr_y - 24.0);
        },
        ScrollDir::HalfUp => {
            w.scroll_to_with_x_and_y(0.0, curr_y - viewport_h);
        },
        ScrollDir::HalfDown => {
            w.scroll_to_with_x_and_y(0.0, curr_y + viewport_h);
        },
        ScrollDir::LineDown => {
            w.scroll_to_with_x_and_y(0.0, curr_y + 24.0);
        },
        ScrollDir::Bottom => {
            w.scroll_to_with_x_and_y(0.0, 9999.9);
        },
    }
}

// fn navigate(nav_option: &NavOpts) {
// 	let nav = use_navigator();
// 	match nav_option {
// 		NavOpts::Home => {
// 			let _ = nav.push(Route::Home {});
// 		}
// 	}
// }
