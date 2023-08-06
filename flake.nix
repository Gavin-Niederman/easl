{
  outputs = { self, nixpkgs }: let pkgs = nixpkgs.legacyPackages.x86_64-linux; in rec {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        pkgconfig
        gdb
        lldb_9
        llvm
        libgccjit
      ];
      LD_LIBRARY_PATH = with pkgs; nixpkgs.lib.makeLibraryPath [
        libgccjit
      ];
      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    };

    overlay = overlays.default;
    overlays.default = (final: _: let  in { easl = import ./default.nix { pkgs = final; };});
  };
}