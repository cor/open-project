{
  description = "A flake for the open-project utility";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "open-project";
      version = "0.1.0";
      src = ./.;

      cargoHash = "sha256-cxy+UZvJNOpfIEZ59d+pT6jbHEpAeOOUjkCaVuT7NBQ=";
    };

    devShell = pkgs.mkShell {
      nativeBuildInputs = [
        pkgs.cargo
        pkgs.rustc
      ];

      shellHook = ''
        echo "Welcome to the open-project development shell!"
      '';
    };
  });
}

