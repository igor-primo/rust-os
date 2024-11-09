{ pkgs, lib, config, inputs, ... }:

{
  packages = [ 
    pkgs.git
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  pre-commit.hooks.rustfmt.enable = true;
}
