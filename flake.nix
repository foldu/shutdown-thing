{
  description = "A thing.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane }: {
    nixosModules.shutdown-thing = import ./nix/module.nix { inherit self; };
  } // flake-utils.lib.eachDefaultSystem
    (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        shutdown-thing-unwrapped =
          let
            cargoToml = builtins.fromTOML (builtins.readFile "${self}/Cargo.toml");
            version = cargoToml.package.version;
            pname = cargoToml.package.name;
            craneLib = crane.lib.${system};
          in
          craneLib.buildPackage {
            inherit pname version;
            src = craneLib.cleanCargoSource (craneLib.path ./.);
          };

        shutdown-thing = pkgs.writeShellScriptBin "shutdown-thing" ''
          PATH=${pkgs.systemd}/bin:$PATH
          exec ${shutdown-thing-unwrapped}/bin/shutdown-thing
        '';
      in
      {
        defaultPackage = shutdown-thing;
        packages = {
          inherit shutdown-thing;
        };
        devShell = pkgs.mkShell { };
      }
    );
}
