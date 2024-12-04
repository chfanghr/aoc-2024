{ inputs, ... }: {
  perSystem = { system, config, ... }:
    let
      rustFlake = inputs.flake-lang.lib.${system}.rustFlake {
        src = ./.;
        crateName = "aoc-2024";
        devShellHook = config.settings.defaultShellHook;
        rustChannel = "nightly";
        extraSourceFilters = [
          (path: _type: builtins.match ".*txt$" path != null)
          (path: _type: builtins.match ".*in$" path != null)
        ];
      };
    in
    {
      inherit (rustFlake) packages checks devShells;
    };
}
