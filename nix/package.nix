{
  lib,
  rustPlatform,
  makeDesktopItem,
  installShellFiles,
  copyDesktopItems,
  makeWrapper,
  samba, # some windows programs require samba
}:

rustPlatform.buildRustPackage {
  pname = "sakaya";
  version = "0.1.0";

  src = builtins.path {
    path = ../.;
    name = "sakaya";
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    installShellFiles
    copyDesktopItems
    makeWrapper
  ];

  postInstall = ''
    installManPage target/man/sakaya.1

    installShellCompletion --cmd sakaya \
      --bash target/completions/sakaya.bash \
      --fish target/completions/sakaya.fish \
      --zsh target/completions/_sakaya

    wrapProgram $out/bin/sakaya \
      --prefix PATH ":" "${
        lib.makeBinPath [
          samba
        ]
      }"
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

  meta = with lib; {
    description = "Run native wine applications inside declarative systemd-nspawn containers";
    homepage = "https://github.com/donovanglover/sakaya";
    license = licenses.mit;
    maintainers = with maintainers; [ donovanglover ];
    mainProgram = "sakaya";
    platforms = platforms.linux;
  };
}
