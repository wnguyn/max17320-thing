## Getting started

Install [Nix](https://nixos.org/download/). The Nix package manager both supports darwin and linux. I don't know what you are gonna do on Windows though.

```sh
nix-shell
```

this gives you a reproducable shell enviroment that...

- has rustc and cargo
- probe-rs tools for flashing


## Build


```sh
cargo build --release --target thumbv7em-none-eabihf
```

## Flash
i'll get these done later...

check that the probe is visible:

```sh
```

flash:

```sh
```
## other
special shoutout to https://github.com/eldruin/max170xx-rs
