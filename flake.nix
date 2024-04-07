{
  description = "Run native wine applications inside declarative systemd-nspawn containers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: {
    packages.x86_64-linux = {
      sakaya = nixpkgs.callPackage ./nix/package.nix { };
      default = nixpkgs.callPackage ./nix/package.nix { };
    };
  };
}
