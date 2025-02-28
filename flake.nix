{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    lpkgs = nixpkgs.legacyPackages.x86_64-linux;
    reqInput = with lpkgs; [
        pkg-config
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr # To use the x11 feature
        xorg.libXinerama
        libxkbcommon
        wayland # To use the wayland feature
        wayland-scanner
        libGL
        libffi
        cmake
    ];
  in {
    devShells.x86_64-linux.default = lpkgs.mkShell {
      buildInputs = reqInput;
      LD_LIBRARY_PATH = with lpkgs; lib.makeLibraryPath reqInput;
      shellHook = ''
        exec zsh
      '';
    };
  };
}
