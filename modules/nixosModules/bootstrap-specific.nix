{ self, ... }: {
  # This file is only only a template
  # bootstrapConfig-specific is replaced by $HOSTNAME-specific in the deployment script using raw string interpolation.
  # Messing with the name WILL break the deployment script.
  flake.nixosModules.bootstrapConfig-specific = { ... }: {
    # Keep empty
  };
}