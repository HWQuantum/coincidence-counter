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
    ((rustChannelOf { date = "2019-06-21"; channel = "nightly"; }).rust.override {extensions = [ "rust-src" ]; })
	    llvmPackages.libclang
      	    hharp
	    (python3.withPackages(ps: with ps; [ pyqtgraph pyqt5 cython numba matplotlib numpy opticspy sympy scipy ]))
	    xorg.libxcb.dev
	  ];
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
    LD_LIBRARY_PATH = "${hharp}/lib";
	}

