{
  description = "A cli utility which makes you remember the God";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    naersk.inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    flake-parts,
    naersk,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {pkgs, ...}: let
        naersk' = pkgs.callPackage naersk {};
      in {
        packages.default = naersk'.buildPackage {
          pname = "whoisgod";
          version = "1.0.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [pkg-config mold];
          buildInputs = with pkgs; [openssl];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-analyzer
            clippy
            openssl
            pkg-config
            mold
          ];
        };
      };
    };
}
