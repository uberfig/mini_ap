{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    openssl.dev
    postgresql
    pkg-config
  ];
  # DATABASE_URL=sqlite:db/analytics.sqlite;
}
