{
  description = "A declarative systemd-nspawn container manager written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: {
    packages.rsc = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
      pname = "rsc";
      version = "0.1.0";
      src = ./.;
      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };

    defaultPackage = self.packages.${system}.rsc;
  });
}
