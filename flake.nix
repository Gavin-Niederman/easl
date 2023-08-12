{
  outputs = { self, nixpkgs }: let pkgs = nixpkgs.legacyPackages.x86_64-linux; in rec {
    devShells.x86_64-linux.default = pkgs.mkShell {
      
    };

    
  };
}

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
        devShells.${system} = import ./shell.nix { inherit pkgs; };
        packages = {
          easl = import ./default.nix;
          default = target;
        };
        apps = rec {
          easl = flake-utils.lib.mkApp { drv = self.packages.${system}.easl; };
          default = winittest;
        };
      }
    )) // rec {
      overlay = overlays.default;
      overlays.default = (final: _: let  in { easl = import ./default.nix; });
    };
  
}