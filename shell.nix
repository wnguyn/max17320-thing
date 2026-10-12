{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    probe-rs-tools
    openocd
  ];

  RUSTUP_TOOLCHAIN = "stable";
}
