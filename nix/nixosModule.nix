self: {
    config,
    lib,
    options,
    pkgs,
    ...
}: let
    cfg = config.services.lumehub;
    format = pkgs.formats.json {};
    mergedSettings = lib.recursiveUpdate options.services.lumehub.settings.default cfg.settings;
    appSettings = format.generate "appsettings.json" mergedSettings;
in {
    options.services.lumehub = {
        enable = lib.mkEnableOption "lumehub";
        dataDir = lib.mkOption {
            type = lib.types.path;
            default = "/var/lib/lumehub";
        };
        openFirewall = lib.mkEnableOption "open firewall for lumehub";
        port = lib.mkOption {
            type = lib.types.port;
            default = 5000;
        };
        settings = lib.mkOption {
            type = format.type;
            default = {
                Logging.LogLevel = {
                    Default = "Information";
                    Microsoft.AspNetCore = "Warning";
                };
                AllowedHosts = "*";
                ConnectionStrings.DatabaseFileName = "LumeHub.Server.db";
                ApiKeySettings = {
                    ApiKey = "MySecretApiKey";
                    ApiKeyHeaderName = "x-api-key";
                    ApiKeySchemeName = "ApiKey";
                };
                LedControllerSettings = {
                    PixelCount = 100;
                    BusId = 0;
                    ClockFrequency = 2000000;
                };
            };
            description = ''
                Specifies the settings passed to the application, which will be merged with the default configuration.
                Only the specified values will override the default values, ensuring that all other settings remain unchanged.
                It is strongly recommended to override the `ApiSettings.ApiKey` value with a secret key.
            '';
        };
    };

    config = lib.mkIf cfg.enable {
        systemd = {
            services.lumehub = {
                description = "lumehub";
                wantedBy = ["multi-user.target"];
                after = ["network.target"];
                serviceConfig = {
                    Type = "simple";
                    ExecStart = "${self.packages.${pkgs.system}.default}/bin/LumeHub.Server --urls http://0.0.0.0:${toString cfg.port}";
                    WorkingDirectory = cfg.dataDir;
                    Restart = "on-failure";
                };
            };
            tmpfiles.rules = [
                # create data directory
                "d ${cfg.dataDir} 0755 root root -"
                # symlink appsettings.json
                "L+ ${cfg.dataDir}/appsettings.json - - - - ${appSettings}"
            ];
        };

        networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [cfg.port];
    };
}
