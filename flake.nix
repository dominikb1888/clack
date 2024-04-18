{
  description = "Small exercises to get you used to reading and writing Rust code";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoBuildInputs = with pkgs; lib.optionals stdenv.isDarwin [
          rustup
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.AppKit
          darwin.apple_sdk.frameworks.WebKit
          darwin.apple_sdk.frameworks.Security
          pkg-config
          openssl
          libiconv
          clang
          llvm
          llvmPackages.libclang
          llvmPackages.llvm
          llvmPackages.bintools
          lld
        ];
     in
      {
        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          buildInputs = with pkgs; [
            rustc
            cargo
            cargo-binutils
            mdbook
            rust-analyzer
            rustfmt
            clippy
            evcxr
            cargo-tauri
            trunk
            wasm-pack
          ] ++ cargoBuildInputs;
        };
     });
}
