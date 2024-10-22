{ config, pkgs, lib, ... }:

let
  inherit (lib) mkEnableOption mkOption mkPackageOption mkIf;
  inherit (lib.types) port string bool;

  cfg = config.sakaya;
in
{
  options.sakaya = {
    enable = mkEnableOption "sakaya server";

    package = mkPackageOption pkgs "sakaya" { };

    openFirewall = mkOption {
      type = bool;
      default = true;
      description = "Whether to automatically open the specified port in the firewall.";
    };

    username = mkOption {
      type = string;
      default = "user";
      description = "The user to run sakaya under.";
    };

    port = mkOption {
      type = port;
      default = 39493;
      description = "The port to listen on for HTTP requests.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.sakaya = {
      enable = true;
      description = "sakaya server";
      unitConfig.Type = "simple";
      path = with pkgs; [ su ];
      serviceConfig.ExecStart = "/usr/bin/env su ${cfg.username} --command='${cfg.package}/bin/sakaya --port ${toString cfg.port}'";
      wantedBy = [ "multi-user.target" ];
    };

    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
  };
}

