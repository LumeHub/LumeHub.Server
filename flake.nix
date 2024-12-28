{
    description = "LumeHub.Server flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        flake-parts.url = "github:hercules-ci/flake-parts";
    };

    outputs = inputs:
        inputs.flake-parts.lib.mkFlake {inherit inputs;} {
            systems = [
                "x86_64-linux"
                "aarch64-linux"
            ];
            flake.nixosModules.default = import ./nix/nixosModule.nix inputs.self;
            perSystem = {pkgs, ...}: let
                dotnet-sdk = pkgs.dotnet-sdk_8;
                aspnetcore = pkgs.dotnet-aspnetcore_8;
            in {
                devShells.default = pkgs.mkShell {
                    packages = [dotnet-sdk];
                };
                packages.default = pkgs.buildDotnetModule {
                    inherit dotnet-sdk;
                    name = "LumeHub.Server";
                    version = "0.1.0";
                    src = ./.;
                    projectFileName = "src/LumeHub.Server/LumeHub.Server.csproj";
                    nugetDeps = ./nix/deps.nix;
                    frameworkDeps = [aspnetcore];

                    makeWrapperArgs = [
                        "--set DOTNET_ROOT ${aspnetcore}"
                    ];
                };
            };
        };
}
