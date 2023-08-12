{ pkgs }:

pkgs.mkShell {
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
}