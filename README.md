# sakaya

Run native wine applications inside declarative systemd-nspawn containers. `sakaya` functions as a replacement for `wine` on the host. Works well with NixOS.

## Features

- Start multiple wine applications that can interact with each other inside sandboxed systemd-nspawn containers
- Automatically open 32/64-bit wine prefixes based on the executable
- Pass-through `/dri` for native GPU performance inside containers
- Prevent sandboxed applications from accessing the internet

## Installation

### [NixOS](https://wiki.nixos.org/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`sakaya`](https://search.nixos.org/packages?channel=unstable&query=sakaya) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    sakaya
  ];
}
```

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install sakaya.

```fish
cargo install --git https://github.com/donovanglover/sakaya --tag 0.1.0
```

### Setup

In order to use sakaya, you must first create a systemd-nspawn container running the sakaya server.

See [`modules/containers.nix`](https://github.com/donovanglover/nix-config/blob/master/modules/containers.nix) in my nix-config for an example.

## Usage

```man
Usage: sakaya [OPTIONS] [FILE] [ARGUMENTS]... [COMMAND]

Commands:
  server  Start a sakaya server instead of a client
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]          Path to the executable to run
  [ARGUMENTS]...  Arguments to pass to [FILE]

Options:
  -a, --address <ADDRESS>      Address of the server to request [default: 0.0.0.0:39493]
  -d, --directory <DIRECTORY>  Host directory mounted to /mnt inside the container [default: /home/user/containers/wine]
  -w, --wine32 <WINE32>        $WINEPREFIX for 32-bit applications (i386) [default: /mnt/wine32]
  -W, --wine64 <WINE64>        $WINEPREFIX for 64-bit applications (amd64) [default: /mnt/wine64]
  -l, --locale <LOCALE>        Locale to run programs with [default: ja_JP.UTF-8]
  -t, --timezone <TIMEZONE>    Timezone to run programs with [default: Asia/Tokyo]
  -f, --force64                Force using the 64-bit $WINEPREFIX for 32-bit applications
  -h, --help                   Print help
  -V, --version                Print version
```

## Contributing

I have been using sakaya since 2023 and it works well for my use case, however suggestions and improvements are welcome. If you would like to contribute code, you can check your work with `cargo clippy`, `cargo fmt`, and `cargo test`.

## Todo

- [x] Reduce compile times
- [x] Notify user if sakaya fails to start an executable
- [x] Choose 32/64-bit automatically based on `IMAGE_FILE_32BIT_MACHINE` in the file headers
- [x] Automatically run winetricks with sane defaults if prefix does not exist already
- [x] Get .ico and convert to .png? Handle abrupt end of files?
- [ ] Write tests
- [x] Terminate sakaya if the server/container cannot be reached
- [x] Create NixOS module to automate systemd service setup for sakaya server
- [x] Automatically register sakaya to executables
- [x] Update format of README
- [ ] Close server connection when terminating client
