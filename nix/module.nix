{ self }: { pkgs, lib, config, ... }:
let
  cfg = config.services.shutdown-thing;
in
{
  services.shutdown-thing = {
    options = with lib; {
      enable = lib.mkEnableOption "shutdown-thing";
      openFirewall = lib.mkOption {
        type = types.bool;
        default = false;
        description = ''
          Opens firewall.
        '';
      };
      addr = lib.mkOptions {
        type = types.string;
        default = "0.0.0.0:5154";
        description = ''
          Addr to run shutdown-thing on.
        '';
      };
    };
    config = lib.mkIf (cfg.enable) {
      users.users.shutdown-thing.isSystemUser = true;
      users.groups.shutdown-thing = { };

      security.sudo.extraRules = [{
        users = [ "shutdown-thing" ];
        commands = builtins.flip builtins.map [ "poweroff" "reboot" "suspend" ] (cmd: {
          command = "${pkgs.systemd}/bin/systemctl ${cmd}";
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
  };
}
