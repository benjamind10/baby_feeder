{
  description = "Rust development environment with Neovim support";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.${builtins.currentSystem};
  in {
    devShell = pkgs.mkShell {
      packages = [
        pkgs.rustup      # Rust toolchain manager
        pkgs.rust-analyzer # Language server for Rust (for Neovim LSP support)
        pkgs.neovim      # Neovim editor
        pkgs.cargo       # Rust's package manager and build system
        pkgs.openssl     # For dependencies requiring SSL (e.g., some Rust crates)
        pkgs.pkgconfig   # For Rust FFI with C libraries
      ];

      shellHook = ''
        # Initialize rustup to add the Rust toolchain
        export RUSTUP_HOME="$PWD/.rustup"
        export CARGO_HOME="$PWD/.cargo"
        rustup default stable
      '';
    };
  };
}
