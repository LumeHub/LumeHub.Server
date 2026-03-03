{self, ...}: {
    flake.nixosModules.default = {
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
                jsonLike = types.recursive (self:
                    types.oneOf (with types; [
                        int
                        float
                        bool
                        str
                        (attrsOf self)
                        (listOf self)
                    ]));

                presetType = types.submodule {
                    options = {
                        effect = mkOption {
                            type = types.str;
                        };
                        params = mkOption {
                            type = types.attrsOf jsonLike;
                            default = {};
                        };
                    };
                };
                mapOptions = mapAttrs (key: value:
                    if key == "presets" && isAttrs value
                    then
                        mkOption {
                            type = types.attrsOf presetType;
                            default = value;
                        }
                    else if isAttrs value
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
    };
}
