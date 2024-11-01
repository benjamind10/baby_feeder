{
  description = "Development environment with Rust, SQLite, OpenSSL, pkg-config, and GCC";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs"; # Use the latest nixpkgs
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in {
      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup       # Rust toolchain manager
          pkgs.sqlite       # SQLite command line tool
          pkgs.openssl      # Needed for many crates
          pkgs.pkg-config   # Helps with linking libraries
          pkgs.gcc
        ];

        # Initializes Rust toolchain within the shell
        shellHook = ''
          rustup default stable
          rustup target add x86_64-unknown-linux-gnu
        '';
      };
    };
}
