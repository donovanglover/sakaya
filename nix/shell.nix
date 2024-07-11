{ pkgs }:

{
  buildInputs = [
    (pkgs.callPackage ./osu.nix { })
  ];
}
