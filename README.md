# LumeHub Server

LumeHub Server is the backend component of the **LumeHub project** — a Rust-based web server that provides a REST API for controlling LED strips.

It supports integration with the **Google Smart Home API**, allowing it to be controlled via **Google Home** devices.
For setup details, see [Google Home integration](#google-home-integration).

---

## Installation

### NixOS

The easiest way to install and run the server is through the **NixOS flake**.
Simply add the project to your flake inputs and import the module:

```nix
{
    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";
        lumehub = {
            url = "github:LumeHub/server";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = {
        nixpkgs,
        lumehub,
        ...
    }: {
        nixosConfigurations.raspberry-pi = nixpkgs.lib.nixosSystem {
            system = "aarch64-linux";
            modules = [
                lumehub.nixosModules.default
                # ... other modules
            ];
        };
    };
}
```

## Configuration

### Nix

You can declaratively configure LumeHub through your NixOS configuration:

```nix
{
    services.lumehub = {
        enable = true;
        openFirewall = true;
        settings = {
            led_controller = {
                controller_type = "ws2801";
                pixel_count = 100;
                spi_path = "/dev/spidev0.0";
                freq_hz = 1000000;
            };
            server = {
                # ip_address = "0.0.0.0"; # alread set when using openFirewall
                port = 5000;
            };
        };
    };
}
```

### TOML

Alternatively, configure the server using a `config.toml` file.

You can place it in the same directory as the lumehub binary, or pass a custom path via the command line:

```bash
lumehub-server --config /path/to/config.toml
# or
lumehub-server -c /path/to/config.toml
```

<details> <summary>Example <code>config.toml</code></summary>

```toml
[led_controller]
controller_type = "ws2801"
pixel_count = 100
spi_path = "/dev/spidev0.0"
freq_hz = 1000000

[server]
ip_address = "0.0.0.0"
port = 5000
```

</details>

## Google Home integration

LumeHub Server supports **Google Home Cloud-to-Cloud** integration.

1. Go to the [Google Home Console](https://console.home.google.com/) and create a new project.
2. Set up **Cloud to Cloud Integration**.
3. Configure the connection to your LumeHub server:
    - **Authorization-URL**: `https://your.domain.com/oauth/authorize`
    - **Token-URL**: `https://your.domain.com/oauth/token`
    - **Cloud-Execution-URL**: `https://your.domain.com/smarthome`
4. Configure your Google Home App:
    - Add a new device
    - Select *Works with Google*
    - Search for your integration name (e.g., *LumeHub*)

> [!NOTE]
> To make this work, you need a domain with a valid SSL certificate that points to your server.

## Development

### Entering the DevShell

Use Nix or Direnv to enter a development environment:

```bash
nix develop
# or
direnv allow
```

This will install all tools required for development and testing.

### Building

```bash
cargo build
# or
nix build
```

### Running / Testing

```bash
cargo run
# or
nix run
```
