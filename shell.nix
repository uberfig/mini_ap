{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Get dependencies from the main package
  # inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  buildInputs = with pkgs; [
    openssl
    openssl.dev
    postgresql
    pkg-config
  ];
  # buildInputs = [ (pkgs.callPackage ./default.nix { }) ];
}