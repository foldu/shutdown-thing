{ self, nixpkgs }: { pkgs, lib, config, ... }:
let
  cfg = config.services.shutdown-thing;
  correctPkgs = nixpkgs.legacyPackages.${pkgs.system};
in
{
  options = with lib; {
    services.shutdown-thing = {
      enable = lib.mkEnableOption "shutdown-thing";
      openFirewall = lib.mkOption {
        type = types.bool;
        default = false;
        description = ''
          Opens firewall.
        '';
      };
      addr = lib.mkOption {
        type = types.str;
        default = "0.0.0.0:5154";
        description = ''
          Addr to run shutdown-thing on.
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
      serviceConfig = {
        Type = "simple";
        User = "shutdown-thing";
        Group = "shutdown-thing";
        Environment = "HOST=${cfg.addr}";
        ExecStart = "${self.packages.${pkgs.system}.shutdown-thing}/bin/shutdown-thing";
      };
    };
  };
}
