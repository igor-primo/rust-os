{ pkgs, lib, config, inputs, ... }:

{
  packages = [ 
    pkgs.rustup
    pkgs.qemu
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

  scripts.build.exec = "cargo build";

  enterShell = ''
    cargo install bootimage
  '';

  enterTest = ''
    cargo test
  '';

  pre-commit.hooks.rustfmt.enable = true;
}
