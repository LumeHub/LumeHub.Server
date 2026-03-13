{
    description = "LumeHub.Server flake";

    nixConfig = {
        extra-substituters = ["https://anders130.cachix.org"];
        extra-trusted-public-keys = ["anders130.cachix.org-1:mCAq0L6Ld3lG7gxJVHGzKr2rqUZ5qs5YoERxoSjMOXs="];
    };

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane.url = "github:ipetkov/crane";
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
                craneLib = inputs.crane.mkLib pkgs;

                serverMeta = craneLib.crateNameFromCargoToml {
                    cargoToml = ./crates/server/Cargo.toml;
                };

                commonArgs = serverMeta // {src = ./.;};

                cargoArtifacts = craneLib.buildDepsOnly commonArgs;

                lumehub-server = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
            in {
                packages.default = lumehub-server;

                checks = {
                    inherit lumehub-server;
                    clippy = craneLib.cargoClippy (commonArgs
                    // {
                        inherit cargoArtifacts;
                        cargoClippyExtraArgs = "-- --deny warnings";
                    });
                    test = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
                };

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
