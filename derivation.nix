{ rustPlatform, fetchgit, lib, pkgs, ... }:

rustPlatform.buildRustPackage {
    pname = "easl";
    version = "0.1.0";

    src = fetchgit {
        url = "https://github.com/Gavin-Niederman/easl";
        sha256 = "sha256-q8d/+ejdjVv1jqvhh9jrhMrCblIf/LyaDG7iIxXddOE=";
    };

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