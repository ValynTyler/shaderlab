{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.build = pkgs.writeShellScriptBin "build" ''
      wasm-pack build --target web
    '';

    packages.x86_64-linux.serve = pkgs.writeShellScriptBin "serve" ''
      xdg-open http://localhost:8000 &
      python3 -m http.server
    '';

    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = [
        # build tools
        pkgs.cargo
        pkgs.rustc
        pkgs.llvmPackages.bintools

        # wasm
        pkgs.wasm-pack

        # host locally
        pkgs.python312
      ];

      shellHook = ''
        echo "Rust WASM development environment"
      '';
    };
  };
}
