{
  description = "A declarative systemd-nspawn container manager written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: {
    packages.sakaya = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
      pname = "sakaya";
      version = "0.1.0";

      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      buildInputs = with nixpkgs.legacyPackages.${system}; [
        openssl
      ];

      nativeBuildInputs = with nixpkgs.legacyPackages.${system}; [
        pkg-config
        installShellFiles
      ];

      postInstall = ''
        installShellCompletion --cmd sakaya \
          --bash <(cat completions/sakaya.bash) \
          --fish <(cat completions/sakaya.fish) \
          --zsh <(cat completions/_sakaya)
      '';
    };

    packages.default = self.packages.${system}.sakaya;
  });
}
