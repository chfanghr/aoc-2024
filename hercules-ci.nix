{ inputs, ... }: {
  imports = [
    inputs.hci-effects.flakeModule
  ];

  herculesCI.ciSystems = [ "x86_64-linux" ];
}
