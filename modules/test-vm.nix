{ inputs, self, ... }: {
  flake.nixosConfigurations.test-vm = inputs.nixpkgs.lib.nixosSystem {
    system = "x86_64-linux";
    modules = [
      self.nixosModules.default
      ({ ... }: {
        services.portfolio.enable = true;

        system.stateVersion = "25.11";
        users.users.root.password = "root";
        services.getty.autologinUser = "root";

        virtualisation.vmVariant = {
          virtualisation.forwardPorts = [
            { from = "host"; host.port = 3000; guest.port = 3000; }
          ];
        };
        networking.firewall.allowedTCPPorts = [ 3000 ];
        systemd.services.portfolio.environment.LEPTOS_SITE_ADDR = "0.0.0.0:3000";
      })
    ];
  };
}