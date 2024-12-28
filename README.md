# LumeHub.Server

## Development

To start developing with the nix development environment, run the following command:

```bash
nix develop
```

Alternatively, you can allow direnv to manage the environment for you:

```bash
direnv allow
```

## Building

To build the project, run the following command:

```bash
dotnet build
```

To build the nix package, run the following command:

```bash
nix build
```

## Running

To run the project, run the following command:

```bash
dotnet run --project src/LumeHub.Server
```

To run the nix package, run the following command:

```bash
cd src/LumeHub.Server
nix run
```

## Testing

To run the tests, run the following command:

```bash
dotnet test
```

To run the nix tests, run the following command:

```bash
nix flake check
```

## Updating Dependencies

To update the nuget dependencies, needed for the nix package, run the following command:

```bash
nix build .#default.passthru.fetch-deps && ./result
```

And then copy the output to `nix/deps.nix`.
