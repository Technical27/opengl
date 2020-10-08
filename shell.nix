let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs {};
  glfw = pkgs.callPackage ./nix/glfw.nix {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    glfw
    xorg.libX11
    libxkbcommon
    libGL
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [ libGL libxkbcommon ]);
}
