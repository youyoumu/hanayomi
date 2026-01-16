{
  description = "A simple Rust + OpenSSL development shell";
  inputs.nixpkgs.url = "nixpkgs";
  outputs =
    { self, nixpkgs }:
    {
      devShells."x86_64-linux".default =
        let
          pkgs = import nixpkgs { system = "x86_64-linux"; };
        in
        pkgs.mkShell {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
            fish -C "
              rustc --version
            "
            exit
          '';
        };
    };
}
