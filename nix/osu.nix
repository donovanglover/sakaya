{
  stdenv,
  fetchzip,
}:

stdenv.mkDerivation {
  name = "osu";
  version = "0-unstable-2023-12-19";

  src = fetchzip {
    url = "https://osekai.net/snapshots/versions/b20231219.2/b20231219.2.zip";
    hash = "sha256-7c1czJFuw+KAr7HoPUBRBieMUtr4hH805UzcEIcD1ok=";
    stripRoot = false;
  };

  postInstall = ''
    install -Dm755 osu!.exe $out/bin/osu!.exe
  '';
}
