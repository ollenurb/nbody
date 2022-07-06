{
    pkgs ? import <nixpkgs> {},
    lib ? pkgs.lib
}:

let

winitDeps = with pkgs; [
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-loader
];

in
pkgs.mkShell {
    buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
    ];

    LD_LIBRARY_PATH = "${lib.strings.makeLibraryPath winitDeps}";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
