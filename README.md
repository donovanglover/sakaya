# sakaya

Run native wine applications inside declarative systemd-nspawn containers. `sakaya` functions as a replacement for `wine` on the host. Works well with NixOS.

## Setup

### Step 1: Add sakaya to your flake inputs

```nix
{
  inputs = {
    sakaya.url = "github:donovanglover/sakaya";
    inputs.nixpkgs.follows = "nixpkgs";
  };
}
```

### Step 2: Create a nixos-container with sakaya

> TODO: NOTE: This part is non-trivial and should be simplified for end-users later.

See [`containers/default.nix`](https://github.com/donovanglover/nix-config/blob/master/containers/default.nix) in my nix-config for an example.

### Step 3: Add sakaya to `systemPackages`

```nix
{ sakaya, pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    sakaya.packages.${pkgs.system}.sakaya
  ];
}
```

## Usage

```man
Usage: sakaya [OPTIONS] [FILE] [COMMAND]

Commands:
  server  Start a sakaya server instead of a client
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  Path to the executable to run

Options:
  -a, --address <ADDRESS>      Address of the server to request [default: 192.168.100.49:39493]
  -d, --directory <DIRECTORY>  Host directory mounted to /mnt inside the container [default: /home/user/containers/wine]
  -w, --wine32 <WINE32>        $WINE_PREFIX for 32-bit applications (i386) [default: /mnt/wine32]
  -W, --wine64 <WINE64>        $WINE_PREFIX for 64-bit applications (amd64) [default: /mnt/wine64]
  -h, --help                   Print help
  -V, --version                Print version
```

## Todo

- [x] Reduce compile times
- [x] Notify user if sakaya fails to start an executable
- [x] Choose 32/64-bit automatically based on `IMAGE_FILE_32BIT_MACHINE` in the file headers
- [ ] Automatically run winetricks based on dll imports?
- [x] Get .ico and convert to .png? Handle abrupt end of files?
- [ ] Write tests
- [ ] Terminate sakaya if the server/container cannot be reached
- [ ] Create NixOS module to automate container creation and configuration?
- [ ] Automatically register sakaya to executables
- [ ] Update format of README
