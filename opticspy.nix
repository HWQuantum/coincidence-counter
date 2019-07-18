{ stdenv, python3Packages, fetchFromGitHub }:
let 
	unwrap = import ./unwrap.nix { stdenv=stdenv; python3Packages=python3Packages; fetchFromGitHub=fetchFromGitHub; };
in
python3Packages.buildPythonPackage rec {
	name = "opticspy";
	version = "0.2.1";
	src = fetchFromGitHub {
		rev = "c818dd64a8451250667d15f7099ba9e9dee410d0";
		sha256 = "1jw6cglsnw0pch8kw4if0sxxahcb87pmrlmpqw17m0cc2ysm0ccw";
		owner = "HWQuantum";
		repo = "opticspy";
	};

	propagatedBuildInputs = with python3Packages; [ unwrap numpy matplotlib cffi ];

	checkPhase = "";
}
