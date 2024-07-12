{ config, pkgs, lib, ... }:

let
  inherit (lib) mkEnableOption mkOption mkPackageOption mkIf singleton;
  inherit (lib.types) port string;

  cfg = config.services.sakaya;
in
{
  options.services.sakaya = {
    enable = mkEnableOption "sakaya server";

    package = mkPackageOption pkgs "sakaya" { };

    openFirewall = mkEnableOption "open port in the firewall needed for sakaya server";

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

