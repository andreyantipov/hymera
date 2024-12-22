{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    # prismaEngines.url = "path:./libs/shared/database/.nix/prisma-engines";
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

              # Common build tools
              pkg-config
              openssl

              # Maintenance
              just
              bacon
            ];

            shellHook = ''
              echo "Hymera development environment"
              echo "Rust version: $(rustc --version)"
              echo "Cargo version: $(cargo --version)"
            '';
          };
        }
      );
    };
}
