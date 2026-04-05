{ ... }: {
  flake.nixosModules.ssh-root = { ... }: {
    services.openssh = {
      enable = true;
      settings.PasswordAuthentication = false;
    };

    users.users.root.openssh.authorizedKeys.keys = [
      "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILDBlVke3NDxu1C61BqmJYwdaOyp/2s6sXScic0mdgWX" # Bootstrap key
    ];
  };
}