let
  nixpkgs =
    let
      rev = "e444e32090f927cbe61a8065a02d9ed666b89228";
      sha256 = "16w94bypdyn02j971cgi71dj707ywx4r33r9rqgyq6a5n5x9sm11";
    in
    builtins.fetchTarball {
      url = "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
      inherit sha256;
    };

  fenix =
    let
      rev = "6fdda0faec538afde79431357ccc87d85c8783f4";
      sha256 = "05wh9zbnic0gq8xlrv1xvls63w07xkk5b6jps1vhvjhz7x1ma13v";
    in
    builtins.fetchTarball {
      url = "https://github.com/nix-community/fenix/archive/${rev}.tar.gz";
      inherit sha256;
    };

  pkgs =
    import nixpkgs {
      config = { };
      overlays = [
        (import "${fenix}/overlay.nix")
      ];
    };

in
pkgs.mkShell {
  packages = with pkgs; [
    pkgs.fenix.latest.cargo
    pkgs.fenix.latest.rustc
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    pkgs.lld
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.AppKit
    pkgs.darwin.apple_sdk.frameworks.ApplicationServices
    pkgs.darwin.apple_sdk.frameworks.CoreVideo
    pkgs.libiconv
  ];
}
