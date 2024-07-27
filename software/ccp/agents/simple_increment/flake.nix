{
  description = "A Rust project that compiles to WebAssembly (WASM) with WASI support using fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      rust = fenix.packages.x86_64-linux.stable.toolchain;
    in
    {
      devShells = {
        x86_64-linux = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.wasmtime
            pkgs.git
            pkgs.htop
          ];

          shellHook = ''
            echo "Development environment for Rust WASM project with WASI support using fenix"
            export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
          '';
        };
      };
    };
}

