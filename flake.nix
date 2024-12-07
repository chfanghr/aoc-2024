{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-lang.url = "github:mlabs-haskell/flake-lang.nix";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    hci-effects.url = "github:hercules-ci/hercules-ci-effects";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
        "aarch64-linux"
      ];

      imports = [
        ./build.nix
        ./pre-commit.nix
        ./settings.nix
        ./hercules-ci.nix
      ];
    };
}
