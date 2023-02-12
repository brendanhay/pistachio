# See: https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-format
{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixpkgs-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, pre-commit-hooks, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = {
          rust = import rust-overlay;
        };

        pkgs = import nixpkgs {
          inherit system;
          overlays = builtins.attrValues overlays;
        };


        # Setup the rust platform binaries.
        # https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md

        rust = pkgs.rust-bin.nightly."2023-01-02".default.override {
          extensions = [
            "rust-src"
            "rust-docs"
            "rust-analyzer-preview"
            "rustfmt-preview"
            "clippy-preview"
          ];
        };

        rust-platform = pkgs.makeRustPlatform {
          rustc = rust;
          cargo = rust;
        };

        rust-workspace = rust-platform.buildRustPackage {
          pname = "arx";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        # Setup git + formatting pre-commit hooks.
        # https://github.com/cachix/pre-commit-hooks.nix

        pre-commit = pre-commit-hooks.lib.${system}.run {
          src = self;
          hooks = {
            nixpkgs-fmt.enable = true;

            clippy = {
              enable = false;
              entry = pkgs.lib.mkForce "${rust}/bin/cargo-clippy";
            };

            rustfmt = {
              enable = true;
              entry = pkgs.lib.mkForce "${rust}/bin/cargo-fmt fmt -- --color always";
            };
          };
        };

      in
      {
        inherit overlays;

        defaultPackage = rust-workspace;

        devShell = pkgs.mkShell {
          name = "pistachio";

          nativeBuildInputs = [
            rust
          ];

          # Install git pre-commit hooks and make the pre-commit binary available on PATH.
          shellHook = pre-commit.shellHook;
        };

        checks = {
          inherit pre-commit;
        };
      });
}
