{ ... }: {
  flake.nixosModules.app-user = { ... }: {
    users.users.app = {
      isNormalUser = true;
      home = "/home/app";
      extraGroups = [ "networkmanager" ];
    };
    users.groups.app = {};
  };
}