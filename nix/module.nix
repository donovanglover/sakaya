{ config, pkgs, lib, ... }:

let
  inherit (lib) mkEnableOption mkOption mkPackageOption mkIf singleton;
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

    address = mkOption {
      type = string;
      default = "192.168.100.49";
      description = "The interface sakaya server will listen on.";
    };

    port = mkOption {
      type = port;
      default = 39493;
      description = "The port to listen on for HTTP requests.";
    };

    noJapanese = mkEnableOption "disable Japanese locale and timezone";
  };

  config = mkIf cfg.enable {
    systemd.services.sakaya = {
      enable = true;
      description = "sakaya server";

      script = ''
        ${cfg.package}/bin/sakaya server \
          --port ${toString cfg.port}
      '';

      serviceConfig = {
        Type = "simple";
        DynamicUser = true;
        Restart = "on-failure";
      };

      wantedBy = [ "multi-user.target" ];
    };

    networking.nat.forwardPorts = singleton {
      destination = "${cfg.address}:${cfg.port}";
      sourcePort = cfg.port;
    };

    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];

    services.xserver.enable = true;

    environment.systemPackages = with pkgs; [
      wineWowPackages.waylandFull
      winetricks
    ];

    environment.sessionVariables = mkIf (!cfg.noJapanese) {
      LC_ALL = "ja_JP.UTF-8";
      TZ = "Asia/Tokyo";
    };
  };
}

