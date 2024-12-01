{
  description = "Ansíne - A lightweight dashboard for home servers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          devShell = with pkgs; mkShell {
            buildInputs = [ cargo rustc rustfmt rustPackages.clippy ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };

          formatter = pkgs.nixpkgs-fmt;
        }) // {
    };
}
