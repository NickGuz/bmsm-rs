{
    description = "Bmsm-rs";
    input.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    outputs = { self, nixpkgs };
    let pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgs;
    in {
        devShells.x86_64-linux.default = pkgs.mkShell {
            name = "Bmsm-rs env";
            packages = with pkgs; [
                rustc
                cargo
                pkg-config
                rust-analyzer
            ];
        };
    };
}
