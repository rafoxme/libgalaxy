{
  description = "LibGalaxy — a GPUI-based Rust GUI library implementing libadwaita widgets with the Ubuntu Yaru theme.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };

          src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            clang
            perl
            python3
            llvmPackages.libclang
          ];

          buildInputs = with pkgs; [
            openssl
            fontconfig
            freetype
            xorg.libxcb
            xorg.libX11
            xorg.libXi
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXScrnSaver
            libxkbcommon
            wayland
            wayland-protocols
            libGL
            vulkan-loader
            dbus
            alsa-lib
            libpulseaudio
            expat
            zlib
          ];
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "libgalaxy-demo";
            version = "0.1.0";
            inherit src nativeBuildInputs buildInputs;

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                # zed monorepo (gpui and friends)
                "collections-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "derive_refineable-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui-0.2.2" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_linux-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_macos-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_macros-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_platform-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_shared_string-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_util-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_web-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_wgpu-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "gpui_windows-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "http_client-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "media-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "perf-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "refineable-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "scheduler-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "sum_tree-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "util_macros-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "zlog-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "ztracing-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";
                "ztracing_macro-0.1.0" = "sha256-oHFcYIWshXQdZP2J936a+bBe4MY55aj50YfMh9E2qz4=";

                # zed-wasm-thread
                "wasm_thread-0.3.3" = "sha256-+lRLCIk0S6Y5ORYjDKsYYHia2FtoSoh+rWkQh7mnPBE=";

                # zed-xim-rs
                "xim-ctext-0.3.0" = "sha256-pRT4Sz1JU9ros47/7pmIW9kosWOGMOItcnNd+VrvnpE=";
                "xim-parser-0.2.1" = "sha256-pRT4Sz1JU9ros47/7pmIW9kosWOGMOItcnNd+VrvnpE=";
                "zed-xim-0.4.0-zed" = "sha256-pRT4Sz1JU9ros47/7pmIW9kosWOGMOItcnNd+VrvnpE=";

                # zed-font-kit
                "zed-font-kit-0.14.1-zed" = "sha256-KXygi0olNQi5yM8eaJVykNDtbPMDjT+cWPBF8UrtXR4=";

                # zed-scap
                "zed-scap-0.0.8-zed" = "sha256-BihiQHlal/eRsktyf0GI3aSWsUCW7WcICMsC2Xvb7kw=";
              };
            };

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";

            buildPhase = ''
              runHook preBuild
              cargo build --release --workspace --bin libgalaxy-demo
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin
              cp target/release/libgalaxy-demo $out/bin/
              runHook postInstall
            '';

            doCheck = false;

            meta = {
              description = "LibGalaxy widget gallery demo";
              license = nixpkgs.lib.licenses.mit;
              platforms = nixpkgs.lib.platforms.linux;
            };
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              pkg-config
              cmake
              clang
              perl
              openssl
              fontconfig
              freetype
              xorg.libxcb
              xorg.libX11
              xorg.libXi
              xorg.libXrandr
              xorg.libXcursor
              xorg.libXScrnSaver
              libxkbcommon
              wayland
              wayland-protocols
              libGL
              dbus
              alsa-lib
              libpulseaudio
            ];
          };
        }
      );
    };
}
