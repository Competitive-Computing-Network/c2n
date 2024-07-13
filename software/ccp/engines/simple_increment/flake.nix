{
  description = "A simple_increment Rust project with WASM support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.wasm-pack
            pkgs.clang
            pkgs.pkg-config
            pkgs.openssl
            pkgs.rustfmt
            pkgs.rustup
            pkgs.wit-bindgen # Correct package name
            pkgs.wasmtime
            pkgs.binaryen
          ];

          shellHook = ''
            rustup default stable
            rustup target add wasm32-unknown-unknown
          '';

          RUSTFLAGS = "--cfg=web_sys_unstable_apis";
        };
      });
}

