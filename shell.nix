
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.neovim                # Neovim
    pkgs.rust-analyzer         # rust-analyzer for Rust LSP
    pkgs.rustc                 # Rust compiler
    pkgs.cargo                 # Cargo package manager
    pkgs.nodejs                # Required for some LSP tools
  ];

  shellHook = ''
    export PATH=$PATH:${pkgs.rustc}/bin:${pkgs.cargo}/bin:${pkgs.rust-analyzer}/bin
  '';
}

