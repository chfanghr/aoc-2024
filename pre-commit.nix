{ inputs, ... }: {
  imports = [
    inputs.pre-commit-hooks.flakeModule
  ];

  perSystem = { config, ... }:
    {
      pre-commit.settings.hooks = {
        nixpkgs-fmt.enable = true;
        deadnix.enable = true;
        rustfmt.enable = true;
        typos.enable = true;
      };

      devShells.dev-pre-commit = config.pre-commit.devShell;
    };
}
