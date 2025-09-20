{
  description = "PlumJam's Website Nix Flake";

  nixConfig = {
    builders-use-substitutes = true;
    flake-registry = "";
    show-trace = true;

    experimental-features = [
      "flakes"
      "nix-command"
      "pipe-operators"
    ];

    extra-substituters = [
      "https://cache.garnix.io/"
      "https://nix-community.cachix.org/"
    ];

    extra-trusted-public-keys = [
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url       = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
		fenix.url       = "github:nix-community/fenix";
		advisory-db     = {
			url   = "github:rustsec/advisory-db";
			flake = false;
		};
  };

  outputs =
    { self, nixpkgs, crane, flake-utils, advisory-db, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
				inherit (pkgs) lib;

				rustCustom = fenix.packages.${system}.combine [
					fenix.packages.${system}.stable.rustc
					fenix.packages.${system}.stable.cargo
					fenix.packages.${system}.stable.clippy
					fenix.packages.${system}.complete.rustfmt
					fenix.packages.${system}.stable.rust-src
					fenix.packages.${system}.stable.rust-analyzer
					fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
				];

        craneLib = (crane.mkLib pkgs).overrideToolchain rustCustom;
				src = lib.cleanSourceWith {
					src = ./.;
					filter = path: type:
						(craneLib.filterCargoSources path type) ||
						(lib.hasSuffix ".css" path) ||
						(lib.hasInfix "/assets/" path);
				};

        commonArgs = {
					inherit src;
					strictDeps = true;
					buildInputs = [];
					CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        tailwindPreBuild = ''
					bunx tailwindcss -i nerd-input.css -o assets/gen-nerd-tailwind.css --minify --content './src/**/*.rs'
					bunx tailwindcss -i normal-input.css -o assets/gen-normal-tailwind.css --minify --content './src/**/*.rs'
        '';

				tailwindBuildInputs = [
					pkgs.bun
					pkgs.tailwindcss_4
				];

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        website = craneLib.buildPackage (
					commonArgs // {
						inherit cargoArtifacts;

						preBuild = tailwindPreBuild;
						nativeBuildInputs = tailwindBuildInputs;

						doCheck = false; # no tests for WASM target
          }
        );
      in
      {
				checks = {
					inherit website;

					rust-clippy = craneLib.cargoClippy (
						commonArgs // {
							inherit cargoArtifacts;

							preBuild = tailwindPreBuild;
							nativeBuildInputs = tailwindBuildInputs;

							cargoClippyExtraArgs = "--all-targets -- --deny warnings";
						}
					);

					rust-doc = craneLib.cargoDoc (
						commonArgs // {
							inherit cargoArtifacts;

							preBuild = tailwindPreBuild;
							nativeBuildInputs = tailwindBuildInputs;

							env.RUSTDOCFLAGS = "--deny warnings";
						}
					);

					rust-fmt = craneLib.cargoFmt {
						inherit cargoArtifacts src;
					};

					# rust-audit = craneLib.cargoAudit {
					#		inherit src advisory-db;
					# };
        };

        packages.default = website;

        apps.default = flake-utils.lib.mkApp {
					drv = website;
				};

        devShells.default = craneLib.devShell {
					# Inherit inputs from checks.
					checks = self.checks.${system};

					buildInputs = [
						pkgs.bun
						pkgs.dioxus-cli
						pkgs.tailwindcss_4
						pkgs.wasm-bindgen-cli
						pkgs.watchman

						pkgs.pkg-config
						pkgs.glib
						pkgs.gtk3
						pkgs.cairo
						pkgs.pango
						pkgs.atk
						pkgs.gdk-pixbuf
						pkgs.libsoup_3
						pkgs.webkitgtk_4_1
					];
				};
			}
		);
}
