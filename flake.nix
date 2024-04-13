#{
#    description = "Bmsm-rs";
#    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#
#    outputs = { self, nixpkgs }:
#    let pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgs;
#    in {
#        devShells.x86_64-linux.default = pkgs.mkShell {
#            name = "Bmsm-rs env";
#            packages = with pkgs; [
#                #rustc
#                #cargo
#                rustup
#                pkg-config
#                #rust-analyzer
#                alsa-lib
#                libudev-zero
#            ];
#        };
#    };
#}

{
  description = "Bmsm-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            alsa-lib
            libudev-zero
            udev
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                extensions = [ 
                  "rust-src" 
                  "rust-analyzer"
                  "rustfmt"
                ];
              })
            )
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}
