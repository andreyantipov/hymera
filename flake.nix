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

        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
                # Development tools
                rustc
                cargo

                # Editor
                zed-editor
                helix

                # Devtools
                rust-analyzer

                # Common build tools & Libs
                openssl
                pkg-config
                atk
                wayland
                fontconfig
                libxkbcommon
                wayland
                libglvnd
                libsoup_3
                webkitgtk_4_1
                xdotool

                # xorg.libX11
                # xorg.libXcursor
                # xorg.libXrandr
                # xorg.libxcb
                # xorg.libXi
            ];

            shellHook = ''
                # envs
                export "LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH"
                export "LD_LIBRARY_PATH=${pkgs.libglvnd}/lib:$LD_LIBRARY_PATH"
                export "LD_LIBRARY_PATH=${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"

                # intro
                echo "Hymera development environment"
                echo "$(rustc --version)"
                echo "$(cargo --version)"
                echo "$(rust-analyzer --version)"

            '';
          };
        }
      );
    };
}
