{ rustPlatform, lib, nix-filter }: rustPlatform.buildRustPackage {
  pname = "tuat-feed-server";
  version = "0.1.0";

  src = nix-filter {
    root = ../.;
    include = [
      "Cargo.toml"
      "Cargo.lock"
      "rust-toolchain.toml"
      "server"
    ];
  };
  cargoLock.lockFile = ../Cargo.lock;

  doCheck = true;

  meta = with lib; {
    description = "A server for parsing and providing json for tuat feed ";
    homepage = "https://github.com/pineapplehunter/tuat-feed-api";
    license = licenses.mpl20;
    maintainers = with maintainers; [ pineapplehunter ];
  };
}
