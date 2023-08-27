# shutdown-thing
Alternative backend for https://apps.somenkov.ru/magic-packet/server/ because I don't like packaging Python

TODO: maybe use polkit + logind instead of sudo

## NixOS module usage
```nix
{
  imports = {
    inputs.shutdown-thing.nixosModules.shutdown-thing
  };
  services.shutdown-thing.enable = true;
}
```
For other options see [./nix/module.nix](./nix/module.nix)

## Binary cache
Latest @ https://app.cachix.org/cache/foldu
