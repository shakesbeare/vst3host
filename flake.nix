{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      reqInputs = [
          pkgs.cmake
          pkgs.pkg-config
          pkgs.xorg.libX11
          pkgs.xorg.libXrandr
          pkgs.xorg.libXinerama
          pkgs.xorg.libXcursor
          pkgs.xorg.libXi
          pkgs.stdenv.cc.cc.lib
      ];
  in
  {
      devShells.x86_64-linux.default = pkgs.mkShell {
          nativeBuildInputs = reqInputs;
          shellHook = "exec zsh";
      };
  };
}
