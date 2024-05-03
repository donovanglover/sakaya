{ config, pkgs, lib, ... }:

with lib;

let
  cfg = config.services.sakaya;
in

{
  options.services.sakaya = {
    enable = mkEnableOption "sakaya server";

    package = mkOption {
      type = types.package;
      default = pkgs.sakaya;
      defaultText = literalExpression "pkgs.sakaya";
      description = "The sakaya package that should be used.";
    };

    openFirewall = mkOption {
      type = types.bool;
      default = false;
      description = "Whether to open ports in the firewall needed for sakaya server to function.";
    };

    port = mkOption {
      type = types.port;
      default = 8080;
      description = "The port to listen on for HTTP requests.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.sakaya = {
      description = "sakaya server";
      wantedBy = [ "multi-user.target" ];
      script = ''
        ${cfg.package}/bin/sakaya server \
          --port ${toString cfg.port}
      '';
      serviceConfig = {
        Type = "simple";
        DynamicUser = true;
        Restart = "on-failure";
      };
    };

    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
  };
}

