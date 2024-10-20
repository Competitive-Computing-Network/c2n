{
  description = "Development environment with Rustup installed via Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";  # Adjust this for your system
      };
    in
    {
      devShells = {
        x86_64-linux = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup       # Rustup for managing Rust toolchains
            pkgs.cmake        # Example: CMake for building C/C++ projects
            pkgs.wasm-pack    # Example: wasm-pack for building Rust WASM projects
            pkgs.wasmtime     # Example: Wasmtime for running WASM modules
            pkgs.wabt         # This adds wasm2wat and other WABT tools
          ];

          shellHook = ''
            echo "Entering development environment with Rustup installed via Nix"
            rustup default stable  # Set default toolchain
            rustup target add wasm32-wasi  # Add WASM target
          '';
        };
      };
    };
}
