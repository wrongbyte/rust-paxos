{
  description = "Environment necessary to run the Paxos implementation.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = {
        rust-bin = import rust-overlay;
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = builtins.attrValues overlays; 
      };
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      run = pkgs.writeShellScriptBin "run" ''
        cargo run --manifest-path paxos/Cargo.toml -- "$@"
      '';
    in 
    with pkgs; {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          rustToolchain
          pkgs.sqlite
          run
        ];

        shellHook = ''
          printf "\nEnvironment is set up! ٩(◕‿◕｡)۶\n
          To run the project, use \`run\`.\n\n"
        }
        '';
      };
    }
  );
}
