{inputs, ...}: {
    imports = [inputs.git-hooks.flakeModule];
    perSystem.pre-commit = {
        check.enable = false;
        settings = {
            enable = true;
            hooks.rustfmt.enable = true;
        };
    };
}
