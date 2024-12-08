{
  description = "Environment necessary to run the Paxos implementation.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.cargo
          pkgs.cargo-watch
          pkgs.sqlite
        ];

        env = {
          DIRENV_LOG_FORMAT="";
        };

        shellHook = ''
          printf "\nEnvironment is set up! ٩(◕‿◕｡)۶\n\n"
        '';
      };
    }
    );
}
