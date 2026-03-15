{self, ...}: {
    flake.nixosModules.default = {
        config,
        lib,
        pkgs,
        ...
    }: let
        inherit (builtins) fromTOML isAttrs isBool isInt mapAttrs readFile;
        inherit (lib) mkEnableOption mkIf mkOption types;
        cfg = config.services.lumehub;

        # Auto-generate typed options from config/default.toml so the NixOS
        # option schema always stays in sync with the server's defaults.
        mapOptions = mapAttrs (_: value:
            if isAttrs value
            then mapOptions value
            else
                mkOption {
                    type =
                        if isBool value
                        then types.bool
                        else if isInt value
                        then types.int
                        else types.str;
                    default = value;
                });

        configFile = pkgs.writers.writeTOML "lumehub-config.toml" (cfg.settings
        // {
            server =
                cfg.settings.server
                // lib.optionalAttrs cfg.openFirewall {
                    ip_address = "0.0.0.0";
                };
        });

        # Resolve a value that is either an inline string or a file path.
        # Paths are used directly (already in the store); strings are written to a file.
        toFile = name: ext: content:
            if builtins.isPath content
            then content
            else pkgs.writeText "lumehub-${name}.${ext}" content;

        # User-supplied effect and function files as a read-only store path.
        # An empty list is valid — the server silently skips missing subdirectories.
        configDir = pkgs.linkFarm "lumehub-config-dir" (
            lib.mapAttrsToList (name: content: {
                name = "effects/${name}.toml";
                path = toFile name "toml" content;
            })
            cfg.extraEffects
            ++ lib.mapAttrsToList (name: content: {
                name = "effects/${name}.rhai";
                path = toFile name "rhai" content;
            })
            cfg.extraEffectsRhai
            ++ lib.mapAttrsToList (name: content: {
                name = "functions/${name}.rhai";
                path = toFile name "rhai" content;
            })
            cfg.extraFunctions
        );
    in {
        options.services.lumehub = {
            enable = mkEnableOption "LumeHub LED controller server";
            package = mkOption {
                type = types.package;
                default = self.packages.${pkgs.stdenv.hostPlatform.system}.lumehub-server.override {
                    inherit (cfg) withGoogle;
                };
                description = "The LumeHub server package to run.";
            };
            openFirewall = mkEnableOption "open firewall port for LumeHub";
            withGoogle = mkOption {
                type = types.bool;
                default = true;
                description = "Include Google Home integration. Disable to produce a smaller binary without Google fulfillment or OAuth.";
            };

            settings = ../config/default.toml |> readFile |> fromTOML |> mapOptions;

            extraEffects = mkOption {
                type = types.attrsOf (types.either types.str types.path);
                default = {};
                description = ''
                    Additional composite effect presets written in TOML.
                    Each key becomes the preset name. The value is either an inline TOML string
                    or a path to a .toml file (e.g. ./effects/my_wave.toml).
                    Extends the built-in stock library — user files with the same name take priority.
                '';
                example = lib.literalExpression ''
                    {
                        # inline
                        my_wave = \'\'
                            [[layers]]
                            effect = "script"
                            speed  = 1.0
                            code   = "from_hue(pixel.to_float() / len.to_float() * 360.0 + time * speed * 60.0)"
                        \'\';
                        # from file
                        ocean_custom = ./lumehub/effects/ocean_custom.toml;
                    }
                '';
            };

            extraEffectsRhai = mkOption {
                type = types.attrsOf (types.either types.str types.path);
                default = {};
                description = ''
                    Additional single-layer script effects written in Rhai.
                    Each key becomes the preset name. The value is either an inline Rhai expression
                    or a path to a .rhai file (e.g. ./effects/my_aurora.rhai).
                    The server automatically wraps each script in a single script layer.
                    Extends the built-in stock library — user files with the same name take priority.
                '';
                example = lib.literalExpression ''
                    {
                        # inline
                        my_aurora = "dim(from_hue(pixel.to_float() / len.to_float() * 360.0), 0.8)";
                        # from file
                        plasma_custom = ./lumehub/effects/plasma_custom.rhai;
                    }
                '';
            };

            extraFunctions = mkOption {
                type = types.attrsOf (types.either types.str types.path);
                default = {};
                description = ''
                    Additional Rhai helper functions prepended to every effect script as a prelude.
                    Each key is the function file stem. The value is either inline Rhai code
                    or a path to a .rhai file (e.g. ./lumehub/functions/my_helpers.rhai).
                    Extends the built-in function library — user files with the same name take priority.
                '';
                example = lib.literalExpression ''
                    {
                        # inline
                        my_helpers = "fn pulse(t, spd) { sin(t * spd) * 0.5 + 0.5 }";
                        # from file
                        color_utils = ./lumehub/functions/color_utils.rhai;
                    }
                '';
            };
        };

        config = mkIf cfg.enable {
            systemd.services.lumehub = {
                description = "LumeHub LED controller server";
                wantedBy = ["multi-user.target"];
                after = ["network.target"];
                serviceConfig = {
                    Type = "simple";
                    ExecStart = "${cfg.package}/bin/lumehub-server --config ${configFile} --config-dir ${configDir} --state-dir $STATE_DIRECTORY";
                    Restart = "on-failure";
                    StateDirectory = "lumehub";
                };
            };

            networking.firewall = mkIf cfg.openFirewall {
                allowedTCPPorts = [cfg.settings.server.port];
                allowedUDPPorts = lib.optional cfg.settings.mdns.enable cfg.settings.mdns.port;
            };
        };
    };
}
