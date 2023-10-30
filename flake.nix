{
  description = "Run native wine applications inside declarative systemd-nspawn containers";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: {
    packages.sakaya = with nixpkgs.legacyPackages.${system}; rustPlatform.buildRustPackage {
      pname = "sakaya";
      version = "0.1.0";

      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      nativeBuildInputs = [
        installShellFiles
        copyDesktopItems
      ];

      postInstall = ''
        installManPage target/man/sakaya.1

        installShellCompletion --cmd sakaya \
          --bash <(cat target/completions/sakaya.bash) \
          --fish <(cat target/completions/sakaya.fish) \
          --zsh <(cat target/completions/_sakaya)
      '';

      desktopItems = [
        (makeDesktopItem {
          name = "sakaya";
          desktopName = "sakaya";
          icon = "wine";
          exec = "sakaya %U";
          mimeTypes = [ "application/x-ms-dos-executable" ];
        })
      ];
    };

    packages.default = self.packages.${system}.sakaya;
  });
}
