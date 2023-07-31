# sakaya

Run native wine applications inside declarative systemd-nspawn containers.

## Installation

### Step 1: Add sakaya to your flake inputs

```nix
{
  inputs = {
    sakaya.url = "github:donovanglover/sakaya";
  };
}
```

### Step 2: Create a declarative systemd-nspawn container with sakaya

See [`sakaya.nix`](https://github.com/donovanglover/nix-config/blob/master/containers/sakaya.nix) in my nix-config for an example.

### Step 3: Add sakaya to systemPackages

```nix
{ sakaya, ... }:

{
  environment.systemPackages = [
    sakaya.packages."x86_64-linux".sakaya
  ];
}
```

## Usage

The point of sakaya is to make it easy to run wine applications inside of declarative systemd-nspawn containers. It functions like a replacement for `wine` on the host.

### Run program.exe in a 32-bit wine prefix:

```fish
sakaya program.exe
```

### Run program.exe in a 64-bit wine prefix:

```fish
sakaya --64 program.exe
```
