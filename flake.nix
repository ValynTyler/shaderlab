{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = [
        # build tools
        pkgs.cargo
        pkgs.rustc

        # wasm
        pkgs.wasm-pack

        # host locally
        pkgs.python312
      ];
    };
  };
}
