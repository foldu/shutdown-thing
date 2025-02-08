{
  description = "A thing.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
    }:
    {
      nixosModules.shutdown-thing = import ./nix/module.nix { inherit self nixpkgs; };
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        shutdown-thing =
          let
            cargoToml = builtins.fromTOML (builtins.readFile "${self}/Cargo.toml");
            version = cargoToml.package.version;
            pname = cargoToml.package.name;
          in
          craneLib.buildPackage {
            inherit pname version;
            src = craneLib.cleanCargoSource (craneLib.path ./.);
          };
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
