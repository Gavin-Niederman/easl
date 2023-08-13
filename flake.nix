{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in with pkgs; rec {
        devShells.${system} = import ./shell.nix;
        packages = rec {
          easl = import ./default.nix;
          default = easl;
        };
        apps = rec {
          easl = flake-utils.lib.mkApp { drv = self.packages.${system}.easl; };
          default = easl;
        };
      }
    )) // rec {
      overlay = overlays.default;
      overlays.default = (final: _: let  in { easl = import ./default.nix; });
    };
}