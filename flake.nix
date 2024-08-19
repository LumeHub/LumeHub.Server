{
    description = "Development Environment Flake";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { flake-utils, nixpkgs, self, ... }:
        flake-utils.lib.eachDefaultSystem (system: let
            pkgs = import nixpkgs {inherit system;};
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
        })
        // {
            nixosModules.default = import ./nix/nixosModule.nix self;
        };
}
