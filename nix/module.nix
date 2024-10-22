{ config, pkgs, lib, ... }:

let
  inherit (lib) mkEnableOption mkOption mkIf;
  inherit (lib.types) port str bool;

  package = pkgs.callPackage ./package.nix { };

  cfg = config.sakaya;
in
{
  options.sakaya = {
    enable = mkEnableOption "sakaya server";

    openFirewall = mkOption {
      type = bool;
      default = true;
      description = "Whether to automatically open the specified port in the firewall.";
    };

    username = mkOption {
      type = str;
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
      serviceConfig.ExecStart = "/usr/bin/env su ${cfg.username} --command='${package}/bin/sakaya server --port ${toString cfg.port}'";
      wantedBy = [ "multi-user.target" ];
    };

    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
  };
}

