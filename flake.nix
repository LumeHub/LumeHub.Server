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
        import-tree.url = "github:vic/import-tree";
    };

    outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} (inputs.import-tree ./nix);
}
