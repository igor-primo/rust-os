{ pkgs, lib, config, inputs, ... }:

{
  packages = [ pkgs.git ];

  languages.rust.enable = true;

  pre-commit.hooks.rustfmt.enable = true;
}
