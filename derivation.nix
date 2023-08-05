{ rustPlatform, fetchgit, lib, ... }:

rustPlatform.buildRustPackage {
    pname = "easl";
    version = "0.1.0";

    src = fetchgit {
        url = "https://github.com/Gavin-Niederman/easl";
        sha256 = "sha256-7SupDDy8xWPdylMwYuFZy3iCdQ9E3E46PvxyKG4uEE8=";
    };

    cargoHash = "sha256-1eWdRy5rP7S9lAiDS61WviHMvFDNu4zKhScGP1JYJmk=";

    meta = with lib; {
        description = "A haskell-like shader language for RGB strips";
        homepage = "https://github.com/Gavin-Niederman/easl";
    };
}