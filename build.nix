{ inputs, ... }: {
  perSystem = { system, config, ... }:
    let
      rustFlake = inputs.flake-lang.lib.${system}.rustFlake {
        src = ./.;
        crateName = "aoc-2024";
        devShellHook = config.settings.defaultShellHook;
        rustChannel = "nightly";
      };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
