{
  description = "Run native wine applications inside declarative systemd-nspawn containers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils }: utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: {
    packages = {
      sakaya = import ./nix/package.nix;

      default = self.packages.${system}.sakaya;
    };
  });
}
