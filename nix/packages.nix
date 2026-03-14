{inputs, ...}: {
    perSystem = {pkgs, ...}: let
        craneLib = inputs.crane.mkLib pkgs;

        serverMeta = craneLib.crateNameFromCargoToml {
            cargoToml = ../crates/server/Cargo.toml;
        };

        commonArgs = serverMeta // {src = ../.;};

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        lumehub-server = pkgs.lib.makeOverridable ({withGoogle ? true}:
            craneLib.buildPackage (commonArgs
            // {inherit cargoArtifacts;}
            // pkgs.lib.optionalAttrs (!withGoogle) {
                cargoExtraArgs = "--no-default-features";
            })) {};
    in {
        packages = {
            inherit lumehub-server;
            default = lumehub-server;
        };
        checks = {
            inherit lumehub-server;
            clippy = craneLib.cargoClippy (commonArgs
            // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "-- --deny warnings";
            });
            test = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
        };
    };
}
