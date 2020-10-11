let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs {};
  glfw = if builtins.getEnv "XDG_SESSION_TYPE" == "wayland" then pkgs.callPackage ./nix/glfw.nix {} else pkgs.glfw;
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    glfw
    xorg.libX11
    valgrind
    gdb
    libxkbcommon
    libGL
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [ libGL libxkbcommon ]);
}
