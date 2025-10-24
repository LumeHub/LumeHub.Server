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
            ];
        };
    };
}
