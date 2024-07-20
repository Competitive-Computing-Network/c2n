{
  description = "A Rust project that compiles to WebAssembly (WASM) with WASI support using fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      rust = fenix.packages.x86_64-linux.stable.toolchain;
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = [
          rust
          pkgs.wasmtime
          pkgs.wasilibc
          pkgs.git        # Additional package
          pkgs.htop       # Additional package
        ];

        shellHook = ''
          echo "Development environment for Rust WASM project with WASI support using fenix"
          export RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'
        '';
      };
    };
}
