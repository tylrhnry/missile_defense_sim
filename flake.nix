{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = import nixpkgs { inherit system; };
        in {
            devShell = pkgs.mkShell {
                nativeBuildInputs = with pkgs; [ pkg-config ];
                buildInputs = with pkgs; [
                    # pkg-config
                    fontconfig

                    cargo
                    rustc
                    clippy
                    rustfmt
                ];

                shellHook = ''
                    export SHELL=$(which zsh)
                    exec zsh
                    echo "Welcome to your missile defense simulator"
                '';
            };
        }
    );
}
