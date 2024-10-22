{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      pre-commit-hooks,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (builtins) attrValues;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs)
          makeRustPlatform
          mkShell
          rust-bin
          writeShellApplication
          ;
        inherit (pkgs.darwin.apple_sdk) frameworks;
        inherit (pkgs.lib) optionalString;
        inherit (pkgs.stdenv) isDarwin;

        rust = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = makeRustPlatform {
          rustc = rust;
          cargo = rust;
        };

        scripts.muchat-watch = writeShellApplication {
          name = "muchat-watch";
          runtimeInputs = with pkgs; [
            cargo-nextest
            cargo-watch

            rust
          ];
          text = ''
            exec cargo watch -s 'cargo fmt && cargo clippy --all && cargo nextest run'
          '';
        };

        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;

          hooks = {
            deadnix.enable = true;
            nixfmt-rfc-style.enable = true;

            rustfmt = {
              enable = true;
              packageOverrides.cargo = rust;
            };
          };
        };

        # Fixes a problem where building on Mac would fail for development.
        shell-patch = optionalString isDarwin ''
          export PATH=/usr/bin:$PATH
        '';

        depsByArch = {
          darwin = with frameworks; [
            AppKit
            ApplicationServices
            CoreFoundation
            CoreGraphics
            CoreText
            CoreVideo
            IOSurface
            Security
          ];
          linux = with pkgs; [
            libxkbcommon
            vulkan-loader
            xorg.libxcb
          ];
        };

        systemDeps = if isDarwin then depsByArch.darwin else depsByArch.linux;
        vulkan = pkgs.vulkan-loader + "/lib";
      in
      {
        checks = {
          inherit pre-commit-check;
        };

        packages = rec {
          default = muchat;

          muchat = rustPlatform.buildRustPackage {
            name = "muchat";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "blade-graphics-0.5.0" = "";
              };
            };
            doCheck = false;
          };
        } // scripts;
        devShells.default = mkShell {
          shellHook = ''
            ${pre-commit-check.shellHook}
            ${shell-patch}

            export LD_LIBRARY_PATH=${vulkan}:$LD_LIBRARY_PATH
          '';

          name = "muchat";

          buildInputs =
            with pkgs;
            [
              (attrValues scripts)

              rust
              cargo-nextest
              cargo-watch
            ]
            ++ systemDeps;
        };
      }
    );
}
