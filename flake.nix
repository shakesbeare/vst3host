{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
      devShells.x86_64-linux.default = pkgs.mkShell {
          packages = with pkgs; [
              llvmPackages.libclang
              clang
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          shellHook = "exec $SHELL";
      };
  };
}
