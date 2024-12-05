{ flake-parts-lib, lib, ... }: {
  options.perSystem = flake-parts-lib.mkPerSystemOption {
    options.settings.defaultShellHook = lib.mkOption {
      type = lib.types.separatedString "\n";
    };
  };

  config.perSystem = { config, system, ... }: {
    settings.defaultShellHook =
      let isDarwin = lib.hasSuffix "darwin" system; in
      ''
        ${
          if isDarwin then
            ''
            export LC_CTYPE=UTF-8
            export LANG=C
            ''
          else
            ''
            export LC_CTYPE=C.UTF-8
            export LC_ALL=C.UTF-8
            export LANG=C.UTF-8
            ''
        }

        ${config.pre-commit.installationScript}
      '';
  };
}
