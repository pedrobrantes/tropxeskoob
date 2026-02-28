{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = [
    pkgs.cargo-edit
    pkgs.cargo-watch
  ];

  enterShell = ''
    rustc --version
    cargo --version
  '';
}
