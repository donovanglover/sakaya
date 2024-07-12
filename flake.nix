{
  description = "Run native wine applications inside declarative systemd-nspawn containers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;

      inherit (pkgs) nixfmt-rfc-style callPackage;
    in
    {
      formatter.x86_64-linux = nixfmt-rfc-style;

      packages.x86_64-linux = {
        osu = callPackage ./nix/osu.nix { };
        sakaya = callPackage ./nix/package.nix { };
        default = callPackage ./nix/package.nix { };
      };

      nixosModules = {
        sakaya = callPackage ./nix/module.nix { };
        default = callPackage ./nix/module.nix { };
      };

      devShells.x86_64-linux.default = pkgs.mkShell (import ./nix/shell.nix {
        inherit pkgs;
      });
    };
}
