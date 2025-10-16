self: {
    config,
    lib,
    pkgs,
    ...
}: let
    inherit (builtins) fromTOML isAttrs isInt mapAttrs readFile;
    inherit (lib) mkEnableOption mkIf mkOption types;
    cfg = config.services.lumehub;
    package = self.packages.${pkgs.system}.default;
    configFile = pkgs.writers.writeTOML "config.toml" cfg.settings;
in {
    options.services.lumehub = {
        enable = mkEnableOption "lumehub";
        openFirewall = mkEnableOption "open firewall for lumehub";
        settings = let
            mapOptions = mapAttrs (key: value:
                if isAttrs value
                then mapOptions value
                else
                    mkOption {
                        type =
                            if isInt value
                            then types.int
                            else types.str;
                        default = value;
                    });
        in
            ../config/default.toml
            |> readFile
            |> fromTOML
            |> mapOptions;
    };
    config = mkIf cfg.enable {
        systemd.services.lumehub = {
            description = "lumehub";
            wantedBy = ["multi-user.target"];
            after = ["network.target"];
            serviceConfig = {
                Type = "simple";
                ExecStart = "${package}/bin/lumehub-server -c ${configFile}";
                Restart = "on-failure";
            };
        };

        services.lumehub.settings = mkIf cfg.openFirewall {
            server.ip_address = "0.0.0.0";
        };
        networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [cfg.settings.server.port];
    };
}
