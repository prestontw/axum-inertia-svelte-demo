{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            pnpm
            nodejs_22
            tailwindcss
            rustup
            just
            watchexec
            # typeshare # install through cargo instead
            nodePackages.svelte-language-server
            nodePackages.typescript-language-server
          ] ++ lib.optionals stdenv.isDarwin [libiconv llvmPackages.bintools];
        };
      });
}


