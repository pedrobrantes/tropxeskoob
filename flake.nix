{
  description = "Export your Skoob bookshelf to JSON and CSV";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustVersion;
          cargo = rustVersion;
        };

        tropxeskoob = rustPlatform.buildRustPackage {
          pname = "tropxeskoob";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
      in
      {
        packages.default = tropxeskoob;
        apps.default = flake-utils.lib.mkApp {
          drv = tropxeskoob;
        };
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.pkg-config
            pkgs.openssl
          ];
        };
      }
    );
}
