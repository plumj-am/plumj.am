{
	description = "James' Website Nix Flake";

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
		nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		fenix.url       = "github:nix-community/fenix";
		crane.url       = "github:ipetkov/crane";
    advisory-db     = {
      url   = "github:rustsec/advisory-db";
      flake = false;
    };
	};

	outputs = { self, advisory-db, crane, nixpkgs, flake-utils, fenix, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs) lib;

				rust   = fenix.packages.${system}.stable.toolchain;
				target = fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std;

				craneLib = (crane.mkLib pkgs).overrideToolchain (pkgs.symlinkJoin {
          name  = "rust-toolchain";
          paths = [ rust target ];
        });

				src = craneLib.cleanCargoSource ./.;

				commonArgs = {
					inherit src;
					strictDeps = true;

          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
				};

				cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # build only the cargo deps
        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          # can't test wasm build
          doCheck = false;
        };

				website = craneLib.buildPackage (
					individualCrateArgs
					// {
						pname = "jamesukiyo";
						cargoExtraArgs = "-p jamesukiyo";
						src = craneLib.cleanCargoSource ./.;
					}
				);

			in
			{
				checks = {
					inherit website;

          rust-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          rust-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          rust-fmt = craneLib.cargoFmt {
            inherit src;
          };

          rust-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
				};

				packages.default = website;

				apps.default = flake-utils.lib.mkApp {
					drv = website;
				};

				devShells.default = craneLib.devShell {
					checks = self.checks.${system};
					packages = [
					  pkgs.dioxus-cli
					];
				};
			}
		);
}
