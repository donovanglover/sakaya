# sakaya

Run native wine applications inside declarative systemd-nspawn containers. `sakaya` functions as a replacement for `wine` on the host. Works well with NixOS.

## Features

- Start multiple wine applications that can interact with each other inside sandboxed systemd-nspawn containers
- Automatically open 32/64-bit wine prefixes based on the executable
- Pass-through `/dri` for native GPU performance inside containers
- Prevent sandboxed applications from accessing the internet

## Installation

### [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`sakaya`](https://search.nixos.org/packages?channel=unstable&query=sakaya) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    sakaya
  ];
}
```

### [Arch Linux](https://archlinux.org/)

```fish
git clone https://github.com/donovanglover/sakaya && cd sakaya && makepkg -si
```

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install sakaya.

```fish
cargo install --git https://github.com/donovanglover/sakaya --tag 0.1.0
```

### Setup

In order to use sakaya, you must first create a nixos-container running the sakaya server.

See [`containers/default.nix`](https://github.com/donovanglover/nix-config/blob/master/containers/default.nix) in my nix-config for an example.

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
  -w, --wine32 <WINE32>        $WINEPREFIX for 32-bit applications (i386) [default: /mnt/wine32]
  -W, --wine64 <WINE64>        $WINEPREFIX for 64-bit applications (amd64) [default: /mnt/wine64]
  -h, --help                   Print help
  -V, --version                Print version
```

## Contributing

sakaya should be bug free, however contributions are welcome. Although NixOS has first-class support, it should be possible to use sakaya with other distributions as well.

## Todo

- [x] Reduce compile times
- [x] Notify user if sakaya fails to start an executable
- [x] Choose 32/64-bit automatically based on `IMAGE_FILE_32BIT_MACHINE` in the file headers
- [ ] Automatically run winetricks based on dll imports?
- [x] Get .ico and convert to .png? Handle abrupt end of files?
- [ ] Write tests
- [x] Terminate sakaya if the server/container cannot be reached
- [ ] Create NixOS module to automate container creation and configuration?
- [ ] Automatically register sakaya to executables
- [x] Update format of README
