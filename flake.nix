{
    description = "LumeHub.Server flake";

    nixConfig = {
        extra-substituters = ["https://anders130.cachix.org"];
        extra-trusted-public-keys = ["anders130.cachix.org-1:mCAq0L6Ld3lG7gxJVHGzKr2rqUZ5qs5YoERxoSjMOXs="];
    };

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        naersk = {
            url = "github:nix-community/naersk/master";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        flake-parts = {
            url = "github:hercules-ci/flake-parts";
            inputs.nixpkgs-lib.follows = "nixpkgs";
        };
    };

    outputs = inputs:
        inputs.flake-parts.lib.mkFlake {inherit inputs;} {
            systems = [
                "x86_64-linux"
                "aarch64-linux"
            ];
            imports = [./nix/nixosModule.nix];
            perSystem = {pkgs, ...}: let
                naersk-lib = pkgs.callPackage inputs.naersk {};
            in {
                packages.default = naersk-lib.buildPackage ./.;
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        cargo
                        rustc
                        rustfmt
                        pre-commit
                        rustPackages.clippy
                    ];
                    RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
                };
            };
        };
}
