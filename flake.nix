{
  description = "Development environment with Rust, React, LSP configurations for Neovim, SQLite, OpenSSL, pkg-config, and GCC";

  inputs = {
    # Use the latest Nix Packages
    nixpkgs.url = "github:NixOS/nixpkgs";
    
    # flake-utils for handling multiple systems and more organized outputs
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            # Rust Development Tools
            pkgs.rustup               # Rust toolchain manager
            pkgs.rust-analyzer        # LSP server for Rust

            # SQLite and Related Tools
            pkgs.sqlite               # SQLite command line tool
            pkgs.openssl              # Needed for many crates
            pkgs.pkg-config           # Helps with linking libraries
            pkgs.gcc                  # GCC compiler

            # Frontend Development Tools
            pkgs.nodejs               # Node.js for React
            pkgs.yarn                 # Yarn package manager
            pkgs.typescript           # TypeScript language support

            # LSP Servers for Frontend
            pkgs.nodePackages.typescript-language-server  # LSP server for TypeScript
            pkgs.nodePackages.eslint                   # Linting tool for JavaScript/TypeScript
            pkgs.nodePackages.prettier                 # Code formatter

            # Optional: Additional Development Tools
            # pkgs.git                # Git for version control (if not already installed)
            # pkgs.npm                # Alternatively, use npm instead of Yarn
          ];

          # Environment Variables (if needed)
          shellHook = ''
            # Initialize Rust toolchain
            rustup default stable
            rustup target add x86_64-unknown-linux-gnu

            # Optional: Initialize Node.js environment (Yarn version, etc.)
            export PATH="$HOME/.cargo/bin:$PATH"
          '';

          # Optional: Additional Environment Setup
          # For example, setting up aliases or environment variables
        };
      }
    );
}

