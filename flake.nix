{
  description = "Rust Development Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/567a49d1913ce81ac6e9582e3553dd90a955875f";
<<<<<<< HEAD
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
=======
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
>>>>>>> 40d0dee (init commit)
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-gnu" ];
        };

        radvIcd = "${pkgs.mesa}/share/vulkan/icd.d/radeon_icd.x86_64.json";
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = [
            rust
            trunk

            openssl
            pkg-config

            libxkbcommon
            libGL
            fontconfig

            wayland

            vulkan-loader
            vulkan-headers
            vulkan-validation-layers

            mesa

<<<<<<< HEAD
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
=======
            libXcursor
            libXrandr
            libXi
            libX11
>>>>>>> 40d0dee (init commit)
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

          shellHook = ''
            export VK_ICD_FILENAMES="${radvIcd}"
          '';
        };
      }
    );
}
