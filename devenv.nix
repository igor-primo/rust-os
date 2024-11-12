{ pkgs, lib, config, inputs, ... }:

{
  packages = [ 
    pkgs.git
    pkgs.rustup
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ 
      "rust-src"
      "cargo"
      "rustc"
      "llvm-tools-preview"
    ];
    targets = [
      "x86_64-unknown-none"
    ];
  };

  pre-commit.hooks.rustfmt.enable = true;
}
