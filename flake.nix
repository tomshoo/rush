{
  description = "Rush a simple programmable shell written in rust";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... } :
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          buildInputs = with pkgs;
            [ openssl
              pkg-config
              rust-bin.stable.latest.default
            ];
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          shellHook =
            ''
            alias build='cargo build'
            alias run='cargo run'
            alias test='cargo test'
            '';
        };

        overlay = new : old : {
          rush = with new; stdnev.mkDerivation {
            inherit buildInputs;

            pname = "rush";
            src = self;

            installPhase =
              ''
              mkdir -p $out/bin
              cargo build --release

              cp target/release/rush $out/bin/
              '';
          };
        };

        defaultPackage = nixpkgs.lib.genAttrs ["x86_64-linux"] (system: (import nixpkgs {
          inherit system;
          overlays = [ self.overlays ];
        }));
      }
  );
}
