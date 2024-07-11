{
  description = "Run native wine applications inside declarative systemd-nspawn containers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs.legacyPackages.x86_64-linux) nixfmt-rfc-style callPackage;
    in
    {
      formatter.x86_64-linux = nixfmt-rfc-style;

      packages.x86_64-linux = {
        osu = callPackage ./nix/osu.nix { };
        sakaya = callPackage ./nix/package.nix { };
        default = callPackage ./nix/package.nix { };
      };
    };
}
