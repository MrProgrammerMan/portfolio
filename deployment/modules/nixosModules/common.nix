{ self, inputs, ... }: {
  flake.nixosModules.common = { config, ... }: {
    imports = with self.nixosModules; [
      bootstrap
      secrets
      inputs.agenix.nixosModules.default
      app-user
      ssh-root
      inputs.portfolio.nixosModules.default
    ];

    services.portfolio.enable = true;

    services.caddy ={
      enable = true; # Should reverse proxy to localhost 3000
      virtualHosts."jonas.baugerud.no".extraConfig = ''
        reverse_proxy http://localhost:3000
      '';
    };

    networking.firewall.allowedTCPPorts = [ 80 443 ];
  };
}