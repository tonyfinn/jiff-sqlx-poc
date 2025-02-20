{
  description = "Jiff SQLx POC";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable."1.81.0".default.override {
              extensions = ["rust-src" "rust-analyzer" "llvm-tools-preview"];
            })
            pkgs.sqlx-cli
            pkgs.postgresql_16
            pkgs.pgcli
            pkgs.openssl
            pkgs.pkg-config
          ];
        };
      }
    );
}
