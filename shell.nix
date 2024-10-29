{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup       # Rust toolchain manager
    pkgs.sqlite       # SQLite command line tool
    pkgs.openssl      # Needed for many crates
    pkgs.pkg-config   # Helps with linking libraries
  ];

  # Initializes Rust toolchain within the shell
  shellHook = ''
    rustup default stable
    rustup target add x86_64-unknown-linux-gnu
  '';
}