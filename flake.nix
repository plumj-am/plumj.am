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
      "https://nix-community.cachix.org/"
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let

        pkgs = import nixpkgs { inherit system; };

        inherit (pkgs) lib;

        tailwindcss = pkgs.tailwindcss_4;
        tailwindCli = "${tailwindcss}/bin/tailwindcss";

        # `nixpkgs` provides rustc >= 1.98, which Topcoat requires. rustfmt
        # comes from fenix's nightly because `.rustfmt.toml` uses unstable
        # options that stable rustfmt silently ignores.
        rustToolchain = pkgs.symlinkJoin {
          name = "rust";
          paths = [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            fenix.packages.${system}.latest.rustfmt # Nightly only for rustfmt.
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            cargoExtraArgs = "--package libs --package normal --package nerd";
          }
        );

        # Topcoat's build script stages Iconify sets; give it a writable cache
        # holding the committed set files instead of the read-only source tree.
        # `TAILWIND_CLI` keeps the Tailwind build offline.
        frontendEnv = {
          nativeBuildInputs = [ tailwindcss ];
          TAILWIND_CLI = tailwindCli;
          preBuild = ''
            export TOPCOAT_ICON_CACHE="$TMPDIR/topcoat-icon-cache"
            mkdir -p "$TOPCOAT_ICON_CACHE"
            cp ${./normal/icons}/*.json "$TOPCOAT_ICON_CACHE"/
          '';
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./libs)
              (craneLib.fileset.commonCargoSources ./normal)
              (craneLib.fileset.commonCargoSources ./nerd)
              ./normal/input.css
              ./normal/icons
              ./normal/assets
              ./normal/posts
              ./nerd/input.css
              ./nerd/assets
            ];
          };

        # A Topcoat server: a native binary plus the static files it serves.
        topcoatApp =
          { pname }:
          craneLib.buildPackage (
            commonArgs
            // frontendEnv
            // {
              inherit cargoArtifacts;
              inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
              inherit pname;

              cargoExtraArgs = "--package ${pname}";
              src = fileSetForCrate (./. + "/${pname}");
              doCheck = false;

              # The server serves these from `$out/share/${pname}/assets`,
              # which the deployment points its `*_ASSETS_DIR` at.
              postInstall = ''
                mkdir -p $out/share/${pname}
                cp -r --no-preserve=mode ${./. + "/${pname}/assets"} $out/share/${pname}/assets
              '';
            }
          );

        libs = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
            pname = "libs";

            cargoExtraArgs = "--package libs";
            src = fileSetForCrate ./libs;
            doCheck = false;
          }
        );

        normal = topcoatApp { pname = "normal"; };

        nerd = topcoatApp { pname = "nerd"; };
      in
      {
        checks =
          lib.mapAttrs' (name: value: lib.nameValuePair "package-${name}" value) self.packages.${system}
          // {
            rust-clippy = craneLib.cargoClippy (
              commonArgs
              // frontendEnv
              // {
                inherit cargoArtifacts;

                src = fileSetForCrate ./.;

                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );

            rust-doc = craneLib.cargoDoc (
              commonArgs
              // frontendEnv
              // {
                inherit cargoArtifacts;

                src = fileSetForCrate ./.;

                env.RUSTDOCFLAGS = "--deny warnings";
              }
            );

            rust-fmt = craneLib.cargoFmt {
              inherit src;

              rustFmtExtraArgs = "--config-path ${./.rustfmt.toml}";
            };

            toml-fmt = craneLib.taploFmt {
              src = lib.sources.sourceFilesBySuffices src [ ".toml" ];

              taploExtraArgs = "--config ${./.taplo.toml}";
            };

            rust-audit = craneLib.cargoAudit {
              inherit src advisory-db;
            };
          };

        packages = {
          inherit libs normal nerd;
          default = normal;
        };

        apps = {
          libs = flake-utils.lib.mkApp {
            drv = libs;
          };
          normal = flake-utils.lib.mkApp {
            drv = normal;
          };
          nerd = flake-utils.lib.mkApp {
            drv = nerd;
          };
          default = flake-utils.lib.mkApp {
            drv = normal;
          };
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          buildInputs = [
            (pkgs.callPackage ./nix/packages/topcoat-cli.nix { })

            tailwindcss
          ];

          TAILWIND_CLI = tailwindCli;
        };
      }
    )
    // {
      # Runs either site as a hardened systemd service:
      #
      #   services.plumjam-website.sites.normal.enable = true;
      #   services.plumjam-website.sites.nerd.enable = true;
      nixosModules.default =
        {
          lib,
          pkgs,
          ...
        }:
        {
          imports = [ ./nix/modules/plumjam-website.nix ];

          services.plumjam-website.sites = {
            normal = {
              package = lib.mkDefault self.packages.${pkgs.system}.normal;
              port = lib.mkDefault 3000;
            };
            nerd = {
              package = lib.mkDefault self.packages.${pkgs.system}.nerd;
              port = lib.mkDefault 3001;
            };
          };
        };
    };
}
