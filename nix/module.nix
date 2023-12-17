{ config, lib, pkgs, ... }: with lib;
let
  cfg = config.pineapplehunter.services.tuat-feed-server;
in
{
  options.pineapplehunter.services.tuat-feed-server = {
    enable = mkEnableOption "Enable the tuat feed server service";

    address = mkOption rec {
      type = types.str;
      default = "0.0.0.0:8000";
      example = default;
      description = "the port to run the server";
    };

    base_url = mkOption {
      type = types.str;
      default = "";
      example = "/tuat";
      description = "the base url to run the server";
    };

    program = mkOption {
      type = types.package;
      default = pkgs.tuat-feed-server;
      description = "the program to run";
    };
  };

  config = {
    systemd.services.tuat-feed-server = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        Restart = "on-failure";
        ExecStart = "${cfg.program}/bin/tuat-feed-server";
        DynamicUser = "yes";
      };
      environment = {
        TUAT_FEED_API_ADDR = cfg.address;
        TUAT_FEED_API_BASEPATH = "/tuat";
      };
    };
  };
}
