# TODO: Remove once in nixpkgs.
{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
let
  inherit (lib.licenses) mit;
  pname = "topcoat-cli";
  version = "0.10.0";
  src = fetchFromGitHub {
    owner = "tokio-rs";
    repo = "topcoat";
    rev = "v${version}";
    hash = "sha256-rbZlsfBHwVqzlzI0TRD7Uq8ril3guyFPCj6UNUnaz68=";
  };
in
rustPlatform.buildRustPackage {
  inherit pname src version;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = "${src}/Cargo.lock";
  };

  # The topcoat workspace also contains other packages.
  # We only need the CLI package.
  cargoBuildFlags = [
    "--package"
    pname
  ];
  cargoInstallFlags = [
    "--path"
    "."
    "--package"
    pname
  ];

  doCheck = false;

  meta = {
    homepage = "https://github.com/tokio-rs/topcoat";
    license = mit;
    mainProgram = "topcoat";
  };
}
