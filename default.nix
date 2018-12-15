with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "papago";
  buildInputs = [
    rustup
    pkgconfig
    openssl
  ];
}
