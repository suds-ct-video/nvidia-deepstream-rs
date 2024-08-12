{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            (fenix.packages.${system}.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer-nightly
            jujutsu

            pkg-config
            # Add additional build inputs here
            glib
            gobject-introspection
            # GStreamer
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
          ]
          ++ pkgs.lib.optional pkgs.stdenv.isDarwin [
            iconv
          ];
      };
    });
}
