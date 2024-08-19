self: {
    config,
    lib,
    pkgs,
    ...
}: let
    cfg = config.services.lumehub;
    format = pkgs.formats.json {};
    appSettings = format.generate "appsettings.json" cfg.settings;
in {
    options.services.lumehub = {
        enable = lib.mkEnableOption "lumehub";
        settings = lib.mkOption {
            type = format.type;
            default = {};
        };
        dataDir = lib.mkOption {
            type = lib.types.path;
            default = "/var/lib/lumehub";
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
                    ExecStart = "${self.packages.${pkgs.system}.default}/bin/LumeHub.Server";
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
    };
}
