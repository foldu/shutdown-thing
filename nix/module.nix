{ self, nixpkgs }: { pkgs, lib, config, ... }:
let
  cfg = config.services.shutdown-thing;
  system = pkgs.system;
  correctPkgs = nixpkgs.legacyPackages.${system};
in
{
  options = with lib; {
    services.shutdown-thing = {
      enable = lib.mkEnableOption "shutdown-thing";
      addr = lib.mkOption {
        type = types.str;
        default = "0.0.0.0";
        description = ''
          Addr to run shutdown-thing on.
        '';
      };
      port = lib.mkOption {
        type = types.port;
        default = 5154;
        description = ''
          Port to run shutdown-thing on.
        '';
      };
    };
  };
  config = lib.mkIf (cfg.enable) {
    users.users.shutdown-thing = {
      isSystemUser = true;
      group = "shutdown-thing";
    };
    users.groups.shutdown-thing = { };

    security.sudo.extraRules = [{
      users = [ "shutdown-thing" ];
      commands = lib.flip builtins.map [ "poweroff" "reboot" "suspend" "is-system-running" ] (cmd: {
        command = "${correctPkgs.systemd}/bin/systemctl ${cmd}";
        options = [ "NOPASSWD" ];
      });
    }];



    systemd.services.shutdown-thing = {
      enable = true;
      description = "Shuts things down.";
      wantedBy = [ "multi-user.target" ];
      environment = {
        ADDR = cfg.addr;
        PORT = toString cfg.port;
        SUDO = "/run/wrappers/bin/sudo";
        SYSTEMCTL = "${correctPkgs.systemd}/bin/systemctl";
      };
      serviceConfig = {
        Type = "simple";
        User = "shutdown-thing";
        Group = "shutdown-thing";
        ExecStart = "${self.packages.${system}.shutdown-thing}/bin/shutdown-thing";
      };
    };
  };
}
