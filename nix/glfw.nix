{ stdenv, fetchFromGitHub, cmake, extra-cmake-modules
, libxkbcommon, wayland, wayland-protocols, libGL
}:

stdenv.mkDerivation rec {
  version = "3.3.2";
  pname = "glfw-wayland";

  src = fetchFromGitHub {
    owner = "glfw";
    repo = "GLFW";
    rev = version;
    sha256 = "0b5lsxz1xkzip7fvbicjkxvg5ig8gbhx1zrlhandqc0rpk56bvyw";
  };

  enableParallelBuilding = true;

  propagatedBuildInputs = [ libGL ];

  nativeBuildInputs = [ cmake extra-cmake-modules wayland-protocols ];

  buildInputs = [ wayland libxkbcommon ];

  cmakeFlags = [ "-DBUILD_SHARED_LIBS=ON" "-DGLFW_USE_WAYLAND=ON" ];
}
