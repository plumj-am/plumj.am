use std::{
    env,
    path::PathBuf,
};

use topcoat::{
    icon::iconify::BuildConfig as IconifyConfig,
    tailwind::BuildConfig as TailwindConfig,
};

fn main() {
    // Render the Tailwind stylesheet. `TAILWIND_CLI` lets a build provide a
    // preinstalled CLI instead of downloading one.
    let tailwind = TailwindConfig::new().input("input.css");
    let tailwind = match env::var_os("TAILWIND_CLI") {
        Some(executable) => tailwind.executable(executable),
        None => tailwind,
    };
    tailwind
        .render()
        .expect("the Tailwind stylesheet should build");

    // Stage the Iconify sets the site renders. `TOPCOAT_ICON_CACHE` points at
    // a writable directory holding the committed set files, so builds never
    // need the network or write into the source tree.
    let cache =
        env::var_os("TOPCOAT_ICON_CACHE").map_or_else(|| PathBuf::from("icons"), PathBuf::from);
    IconifyConfig::new()
        .cache_dir(cache)
        .icon_set("lucide")
        .icon_set("simple-icons")
        .stage()
        .expect("the Iconify sets should stage");
}
