{
  outputs = { self, nixpkgs }: let pkgs = nixpkgs.legacyPackages.x86_64-linux; in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        pkgconfig
        gdb
        lldb_9
        llvm
      ];

      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    };

    overlay = overlays.default;
    overlays.default = final: _: easl = import ./default.nix { pkgs = final; };
  };
}