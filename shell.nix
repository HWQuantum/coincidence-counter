let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz); 
in
    with import <nixpkgs> { overlays = [ moz_overlay ]; };
    let hharp = callPackage ./hydraHarpLib.nix {};
        opticspy = callPackage ./opticspy.nix {};
  in
	stdenv.mkDerivation {
	  name = "rust-env";
	  buildInputs = [
    ((rustChannelOf { date = "2019-10-16"; channel = "nightly"; }).rust.override {extensions = [ "rust-src" ]; })
	    llvmPackages.libclang
      hharp
			(python3.buildEnv.override {
			  extraLibs = with python3Packages; [ pyqtgraph pyqt5 cython numba matplotlib numpy opticspy sympy scipy ];
			  ignoreCollisions = true;
			})
	    xorg.libxcb.dev
	  ];
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
    LD_LIBRARY_PATH = "${hharp}/lib";
	}

