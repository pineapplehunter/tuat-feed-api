{
  description = "A very basic flake";

  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.flake-utils.follows = "flake-utils";
  };
  inputs.nix-filter.url = "github:numtide/nix-filter";

  outputs = { self, nixpkgs, systems, ... }@inputs: {
    overlays.default = final: prev: {
      inherit (self.packages.${final.system or final.stdenv.system}) tuat-feed-server;
    };

    nixosModules.default = ./nix/module.nix;
  } //
  inputs.flake-utils.lib.eachDefaultSystem
    (system:
      let
        genPkgs = crossSystem: import nixpkgs {
          localSystem = system;
          inherit crossSystem;
          overlays = [
            inputs.rust-overlay.overlays.default
            inputs.nix-filter.overlays.default
            self.overlays.default
          ];
        };
        pkgs = genPkgs system;
        lib = nixpkgs.lib;
        forEachSystem = systems: f: lib.genAttrs systems (system: f system);
        allSystems = (import systems) ++ [ "riscv64-linux" ];
      in
      {
        packages = rec{
          default = tuat-feed-server;
          tuat-feed-server = pkgs.callPackage ./nix/package.nix { };
          docker = pkgs.callPackage ./nix/docker.nix { };

          pkgsCross = forEachSystem (lib.filter (sys: sys != system) allSystems) (crossSystem:
            let
              pkgs = genPkgs crossSystem;
            in
            rec {
              default = tuat-feed-server;
              tuat-feed-server = pkgs.callPackage ./nix/package.nix { };
              docker = pkgs.callPackage ./nix/docker.nix { };
            });
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs;[ stdenv.cc pkg-config openssl ];
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
