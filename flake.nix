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
          nativeBuildInputs = with pkgs; [
              pkg-config
              xorg.libX11
              xorg.libXrandr
              xorg.libXinerama
              xorg.libXcursor
              xorg.libXi
          ];
          shellHook = "exec zsh";
      };
  };
}
