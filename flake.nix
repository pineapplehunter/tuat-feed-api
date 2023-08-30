{
  description = "A very basic flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.nix-filter.url = "github:numtide/nix-filter";

  outputs = { self, nixpkgs, flake-utils, rust-overlay, nix-filter }: {
    overlays.default = final: prev: {
      tuat-feed-server = let 
      filter = nix-filter.lib;
      in final.rustPlatform.buildRustPackage rec {
        pname = "tuat-feed-server";
        version = "0.1.0";

        src = filter {
          root= ./.;
          include = [
            ./Cargo.toml
            ./Cargo.lock
            ./rust-toolchain.toml
            "server"
            "common"
            "client"
          ];
        };
        cargoLock.lockFile = ./Cargo.lock;

        doCheck = true;

        meta = with final.lib; {
          description = "A server for parsing and providing json for tuat feed ";
          homepage = "https://github.com/pineapplehunter/tuat-feed-api";
          license = licenses.mpl20;
          maintainers = [ ];
        };
      };
    };

    nixosModules.default = { config, lib, pkgs, ... }: with lib;
      let
        cfg = config.pineapplehunter.services.tuat-feed-server;
      in
      rec {
        options.pineapplehunter.services.tuat-feed-server = {
          enable = mkEnableOption "Enable the tuat feed server service";

          address = mkOption rec {
            type = types.str;
            default = "0.0.0.0:8000";
            example = default;
            description = "the port to run the server";
          };

          base_url = mkOption rec {
            type = types.str;
            default = "";
            example = "/tuat";
            description = "the base url to run the server";
          };

          program = mkOption rec {
            type = types.package;
            default = pkgs.tuat-feed-server;
            description = "the program to run";
          };
        };

        config = {
          nixpkgs.overlays = [ self.overlays.default ];

          systemd.services.tuat-feed-server = {
            wantedBy = [ "multi-user.target" ];
            serviceConfig = {
              Restart = "on-failure";
              ExecStart = "${cfg.program}/bin/tuat-feed-server";
              DynamicUser = "yes";
            };
            environment = {
              TUAT_FEED_API_ADDR = cfg.address;
              TUAT_FEED_API_BASEPATH = "/tuat";
            };
          };
        };
      };
  } //
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; overlays = [ self.overlays.default rust-overlay.overlays.default ]; };
    in
    {
      packages = {
        inherit (pkgs) tuat-feed-server;
        default = self.packages.${system}.tuat-feed-server;
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs;[ stdenv.cc pkg-config openssl ];
      };

      checks = {
        vmTestServerUp =
          with import (nixpkgs + "/nixos/lib/testing-python.nix") { inherit system; };
          makeTest {
            name = "vmTestServerUp";

            nodes.server = { ... }: {
              imports = [ self.nixosModules.default ];
              pineapplehunter.services.tuat-feed-server.enable = true;
            };

            testScript = ''
              start_all()
              server.wait_for_open_port(8000)
            '';
          };
      };

      formatter = pkgs.nixpkgs-fmt;
    });
}
