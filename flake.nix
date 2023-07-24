{
  description = "A declarative systemd-nspawn container manager written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: {
    packages.rsc-client = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
      pname = "rsc-client";
      version = "0.1.0";
      src = ./client;
      cargoLock = {
        lockFile = ./client/Cargo.lock;
      };
    };

    packages.rsc-server = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
      pname = "rsc-server";
      version = "0.1.0";
      src = ./server;
      cargoLock = {
        lockFile = ./server/Cargo.lock;
      };
    };

    defaultPackage = self.packages.${system}.rsc-client;
  });
}
