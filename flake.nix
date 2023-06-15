{
  description = "Rush a simple programmable shell written in rust";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-23.05";
      follows = "rust-overlay/nixpkgs";
    };
  };

  outputs = { nixpkgs,
              rust-overlay,
              flake-utils,
              ...
  } : flake-utils.lib.eachDefaultSystem (system : let
      pkgconf = {
        overlays = [ (import rust-overlay) ];
        localSystem.system = system;
      };
      pkgs = import nixpkgs (pkgconf);
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "rush";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        buildInputs = with pkgs;
          [ openssl
            pkg-config
          ];
      };

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs;
          [ rust-bin.stable."1.70.0".default
            pkg-config
            openssl
          ];

        shellHook =
          ''
          alias build='cargo build'
          alias run='cargo run'
          alias test='cargo test'
          '';
      };
    });
}
