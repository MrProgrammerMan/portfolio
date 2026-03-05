{ inputs, self, ... }: {
  # VERY IMPORTANT: "bootstrapConfig" used with raw string interpolation in the deployment script.
  # DO NOT CHANGE THE NAME OF THIS CONFIGURATION UNLESS YOU ALSO CHANGE IT IN THE DEPLOYMENT SCRIPT.
  # MAKE SURE YOU UNDERSTAND THE DEPLOYMENT SCRIPT BEFORE CHANGING THIS NAME.
  flake.nixosConfigurations.bootstrapConfig = inputs.nixpkgs.lib.nixosSystem {
    system = "x86_64-linux";
    modules = with self.nixosModules; [
      bootstrap
      bootstrapConfig-specific # Yes this is jank, see the bootstrap configuration.nix or readme for more details
      disk-config
      ./_hardware-configuration.nix
      {
        networking.hostName = "bootstrapConfig";
      }
    ];
  };
}