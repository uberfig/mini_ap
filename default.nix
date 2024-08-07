{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    openssl.dev
    postgresql
    pkg-config
  ];
}
