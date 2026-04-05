{ inputs, self, ... }: {
  flake.nixosModules.default = { config, lib, pkgs, ... }:
    let
        cfg = config.services.portfolio;
    in {
      options.services.portfolio = {
        enable = lib.mkEnableOption "portfolio";
        package = lib.mkOption {
          type = lib.types.package;
          default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
          description = "The package to use for the portfolio.";
        };
      };
      config = lib.mkIf cfg.enable {
        systemd.services.portfolio = {
          description = "portfolio";
          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];

          serviceConfig = {
            ExecStart = lib.getExe cfg.package;
            Restart = "on-failure";
            DynamicUser = true;
          };
        };
      };
    };
}