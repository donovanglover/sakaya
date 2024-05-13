{ config, pkgs, lib, ... }:

let
  inherit (lib) mkEnableOption mkOption mkIf literalExpression;
  inherit (lib.types) package port;

  cfg = config.services.sakaya;
in

{
  options.services.sakaya = {
    enable = mkEnableOption "sakaya server";

    package = mkOption {
      type = package;
      default = pkgs.sakaya;
      defaultText = literalExpression "pkgs.sakaya";
      description = "The sakaya package that should be used.";
    };

    openFirewall = mkEnableOption "open port in the firewall needed for sakaya server";

    port = mkOption {
      type = port;
      default = 39493;
      description = "The port to listen on for HTTP requests.";
    };

    noJapanese = mkEnableOption "disable Japanese locale and timezone";
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

    environment.sessionVariables = mkIf (!cfg.noJapanese) {
      LC_ALL = "ja_JP.UTF-8";
      TZ = "Asia/Tokyo";
    };
  };
}

