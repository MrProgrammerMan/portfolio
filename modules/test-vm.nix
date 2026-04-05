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
            { from = "host"; host.port = 443; guest.port = 443; }
            { from = "host"; host.port = 80; guest.port = 80; }
          ];
        };
        networking.firewall.allowedTCPPorts = [ 443 80 ];
        services.caddy ={
          enable = true; # Should reverse proxy to localhost 3000
          virtualHosts."localhost".extraConfig = ''
            tls internal
            reverse_proxy http://localhost:3000
          '';
        };
      })
    ];
  };
}