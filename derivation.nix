{ rustPlatform, fetchgit, lib, ... }:

rustPlatform.buildRustPackage {
    pname = "easl";
    version = "0.1.0";

    src = fetchgit {
        url = "https://github.com/Gavin-Niederman/easl";
        sha256 = "sha256-Z3SjAlSALL399LWdwMP4rX20N9pFYGMSBf4KfcLJZpE=";
    };

    cargoHash = "sha256-kgph+nh0Wva0uJJl0921MAlUugEcXLhqwDTkDfY/5nk=";

    meta = with lib; {
        description = "A haskell-like shader language for RGB strips";
        homepage = "https://github.com/Gavin-Niederman/easl";
    };
}