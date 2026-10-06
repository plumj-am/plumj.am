{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib.attrsets)
    attrValues
    filterAttrs
    mapAttrs'
    nameValuePair
    attrNames
    ;
  inherit (lib.lists) allUnique;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkEnableOption mkOption mkPackageOption;
  inherit (lib.types)
    attrsOf
    nullOr
    path
    port
    str
    submodule
    ;

  cfg = config.services.plumjam-website;

  site = { name, ... }: {
    options = {
      enable = mkEnableOption "the PlumJam `${name}` site";

      package = mkPackageOption pkgs name { default = null; };

      host = mkOption {
        type = str;
        default = "127.0.0.1";
        description = "Address the server binds to.";
      };

      port = mkOption {
        type = port;
        default = 3000;
        description = "TCP port the server binds to.";
      };

      assetsDir = mkOption {
        type = nullOr path;
        default = null;
        description = ''
          Directory of static files served below `/assets`. Defaults to the
          package's `$out/share/<name>/assets`, and is passed to the service as
          `PLUMJAM_ASSETS_DIR`.
        '';
      };

      stateDir = mkOption {
        type = str;
        default = "/var/lib/plumjam-website/${name}";
        description = "Directory for runtime state.";
      };

      environmentFile = mkOption {
        type = nullOr path;
        default = null;
        description = "Systemd EnvironmentFile with the site's secrets.";
      };
    };
  };

  enabled = filterAttrs (_: site: site.enable) cfg.sites;
in
{
  options.services.plumjam-website = {
    user = mkOption {
      type = str;
      default = "plumjam-website";
      description = "User the sites run as.";
    };

    group = mkOption {
      type = str;
      default = "plumjam-website";
      description = "Group the sites run as.";
    };

    sites = mkOption {
      type = attrsOf (submodule site);
      default = { };
      description = "PlumJam sites to run, keyed by name.";
    };
  };

  config = mkIf (attrValues cfg.sites != [ ] && enabled != { }) {
    assertions = [
      {
        assertion = allUnique (map (site: "${site.host}:${toString site.port}") (attrValues enabled));
        message = "services.plumjam-website: enabled sites must not share a host:port.";
      }
    ];

    users.users.${cfg.user} = {
      description = "PlumJam website service user";
      inherit (cfg) group;
      isSystemUser = true;
    };
    users.groups.${cfg.group} = { };

    systemd.services = mapAttrs' (
      name: site:
      let
        assetsDir =
          if site.assetsDir != null then site.assetsDir else "${site.package}/share/${name}/assets";
      in
      nameValuePair "plumjam-website-${name}" {
        description = "PlumJam ${name} site";
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];

        serviceConfig = {
          Type = "simple";
          ExecStart = "${site.package}/bin/${name}";
          Restart = "always";
          RestartSec = "5";

          WorkingDirectory = site.stateDir;
          Environment = [
            "PLUMJAM_ASSETS_DIR=${assetsDir}"
            "HOST=${site.host}"
            "PORT=${toString site.port}"
          ];
          EnvironmentFile = mkIf (site.environmentFile != null) site.environmentFile;

          NoNewPrivileges = true;
          PrivateTmp = true;
          ProtectSystem = "strict";
          ProtectHome = true;
          ProtectKernelTunables = true;
          ProtectKernelModules = true;
          ProtectControlGroups = true;

          User = cfg.user;
          Group = cfg.group;
        };
      }
    ) enabled;

    # The sites only read their assets, so the state dir just needs to exist
    # for `WorkingDirectory`.
    systemd.tmpfiles.rules = map (
      name: "d ${enabled.${name}.stateDir} 0755 ${cfg.user} ${cfg.group} -"
    ) (attrNames enabled);
  };
}
