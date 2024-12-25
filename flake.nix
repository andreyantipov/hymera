{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs = { systems, nixpkgs, ... }@inputs:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      devShells = eachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
          };
        in {
          default = pkgs.mkShell {
            # Tools needed *at build time* (compilers, pkg-config, cargo, editors, etc.)
            nativeBuildInputs = with pkgs; [
              # Native libs
              rustup
              cargo-binstall
              gcc
              pkg-config

            ];

            # Libraries needed to *link* or run your application
            buildInputs = with pkgs; [
              # Editor
              vscode
              helix

              # Devtools

              # Libs 
              glibc
              openssl
              atk
              wayland
              gtk3
              pango
              glib
              zlib
              cairo
              harfbuzz
              fontconfig
              libxkbcommon
              libglvnd
              libsoup_3
              webkitgtk_4_1
              xdotool
            ];

            shellHook = ''
              # If you want to set a default toolchain:
              rustup default stable

              # Example library path exports (sometimes not necessary)
              export "LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH"
              export "LD_LIBRARY_PATH=${pkgs.libglvnd}/lib:$LD_LIBRARY_PATH"
              export "LD_LIBRARY_PATH=${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"

              echo "Hymera development environment:"
              echo "rustc:            $(rustc --version)"
              echo "cargo:            $(cargo --version)"
              echo "rust-analyzer:    $(rust-analyzer --version)"
              echo "dioxus-cli (dx):  $(dx --version)"
            '';
          };
        }
      );
    };
}
