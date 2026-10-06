use std::env;

use topcoat::tailwind::BuildConfig;

fn main() {
    // Render the Tailwind stylesheet. `TAILWIND_CLI` lets a build provide a
    // preinstalled CLI instead of downloading one.
    let tailwind = BuildConfig::new().input("input.css");
    let tailwind = match env::var_os("TAILWIND_CLI") {
        Some(executable) => tailwind.executable(executable),
        None => tailwind,
    };
    tailwind
        .render()
        .expect("the Tailwind stylesheet should build");
}
