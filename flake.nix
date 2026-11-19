{
  description = "A fast, lightweight, and modern Windows 11 Fluent-style text editor built with Rust and Iced";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeLibs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "notepad";
          version = "1.0.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          buildInputs = runtimeLibs;

          postInstall = ''
            wrapProgram $out/bin/notepad \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath runtimeLibs}"
          '';

          meta = with pkgs.lib; {
            description = "A fast, lightweight, and modern text editor built with Rust and Iced";
            homepage = "https://github.com/deactivated0/Notepad";
            license = licenses.mit;
            mainProgram = "notepad";
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
            clippy
            rustfmt
          ];
          buildInputs = runtimeLibs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };
      }
    );
}
