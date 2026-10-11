{ pkgs, lib, ... }:
{
  # Languages
  languages.rust = {
    enable = true;
    lsp.enable = true;
    mold.enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  packages = with pkgs; [
    wayland
    libxkbcommon
    libGL
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  env.LD_LIBRARY_PATH = "${lib.makeLibraryPath (with pkgs; [
    wayland
    libxkbcommon
    libGL
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ])}:/run/opengl-driver/lib";
}

