{ rustPlatform, fetchgit, lib, pkgs, ... }:

rustPlatform.buildRustPackage {
    pname = "easl";
    version = "0.1.0";

    src = ./.;

    cargoHash = "sha256-1GuZHqJz0E927kKA0xtJesS0ebhNFLa5WaBiKcWM76Q=";

    buildInputs = with pkgs; [
        pkgconfig
        gdb
        lldb_9
        llvm
        libgccjit
    ];

    meta = with lib; {
        description = "A haskell-like shader language for RGB strips";
        homepage = "https://github.com/Gavin-Niederman/easl";
    };
}