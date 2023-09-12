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

> LATER: Implement this

You can open applications like normal and they'll open with sakaya. Use `~/.config/sakaya/config.toml` to change whether an application should use a 32 or 64-bit prefix, which container it should run in, etc.

```fish
sakaya program.exe
```

## Todo

- [x] Reduce compile times
- [ ] Notify user if sakaya fails to start an executable
- [ ] Choose 32/64-bit automatically based on `IMAGE_FILE_32BIT_MACHINE` in the file headers
- [ ] Automatically run winetricks based on dll imports?
- [ ] Get .ico and convert to .png? Handle abrupt end of files?
- [ ] Write tests
