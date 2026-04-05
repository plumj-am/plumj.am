#![expect(clippy::unwrap_used, clippy::indexing_slicing, reason = "Build script")]

use std::{
    env,
    fs,
    path::Path,
};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap();
    let cargo_toml = fs::read_to_string(workspace_root.join("Cargo.toml"))
        .expect("Failed to read workspace Cargo.toml");

    let parsed: toml::Value = cargo_toml.parse().expect("Failed to parse Cargo.toml");
    let version = parsed["workspace"]["metadata"]["version"]
        .as_str()
        .expect("workspace.metadata.version not found");

    println!("cargo:rustc-env=APP_VERSION={version}");
}
