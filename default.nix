{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "bayou";
  version = "0.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  nativeBuildInputs = with pkgs; [
    openssl
    openssl.dev
    postgresql
    pkg-config
  ];
  buildInputs = with pkgs; [
    openssl
    openssl.dev
    postgresql
    pkg-config
  ];
}
