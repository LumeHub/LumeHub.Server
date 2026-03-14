{
    perSystem = {pkgs, ...}: {
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
}
