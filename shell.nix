let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz); 
in
    with import <nixpkgs> { overlays = [ moz_overlay ]; };
    let hharp = callPackage ./hydraHarpLib.nix {};
  in
	stdenv.mkDerivation {
	  name = "rust-env";


	  buildInputs = [
	    rustChannels.nightly.rust
	    rustChannels.nightly.cargo
	    llvmPackages.libclang
      	    hharp
	    (python3.withPackages(ps: with ps; [ pyqtgraph pyqt5 ]))
	  ];
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
    LD_LIBRARY_PATH = "${hharp}/lib";
	}

