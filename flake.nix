{
  description = "Development environment for a Rust project";

  inputs = {
    # Pull nixpkgs for base packages
    nixpkgs.url = "github:NixOS/nixpkgs";
    # Optional: Use rust-overlay for managing Rust versions
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }: let
    pkgs = import nixpkgs {
      overlays = [ rust-overlay.overlay ]; # Add Rust overlay
      system = "x86_64-linux"; # Adjust for your architecture
    };

    rustPlatform = pkgs.rustPlatform; # Access Rust tooling
  in {
    # Define the development shell
    devShells.default = pkgs.mkShell {
      name = "rust-dev-shell";

      # Dependencies for the development environment
      buildInputs = [
        rustPlatform.rustc    # Rust compiler
        rustPlatform.cargo    # Cargo package manager
        rustPlatform.rustfmt  # Rust code formatter
        rustPlatform.clippy   # Code linter
        pkgs.openssl          # OpenSSL for crates requiring it
        pkgs.pkg-config       # For linking C libraries
      ];

      # Optional: Shell hook to display helpful messages
      shellHook = ''
        echo "Welcome to the Rust development environment!"
        echo "Rust version: $(rustc --version)"
        echo "Cargo version: $(cargo --version)"
      '';
    };
  };
}

