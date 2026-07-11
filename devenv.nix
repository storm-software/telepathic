{
  pkgs,
  inputs,
  config,
  ...
}:
{
  name = "storm-software/telepathic";

  cachix = {
    push = "github-telepathic";
    pull = [ "github-telepathic" ];
  };

  overlays = [
    inputs.sccache.overlays.default
  ];

  dotenv.enable = true;
  dotenv.filename = [
    ".env"
    ".env.local"
  ];
  dotenv.disableHint = true;

  packages = with pkgs; [
    sccache
    libclang
  ];

  env = {
    # bindgen loads libclang at build time; point it at nix libclang, not host /usr/lib.
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  };

  languages.c.enable = true;

  scripts = {
    build-native.exec = "pnpm build-native --target=$1";
    codegen-lang.exec = "pnpm codegen-lang";
    new-lang.exec = "pnpm new-lang $1 $2";
  };

  profiles = {
    debug = {
      extends = [
        "development"
      ];
      module = {
        env = {
          SCCACHE_ERROR_LOG = "${config.git.root}/tmp/sccache.log";
          SCCACHE_ENDPOINT = "https://rustc-cache.storm-cdn.com/telepathic";
          SCCACHE_BUCKET = "rustc-cache";
          RUST_BACKTRACE = "1";
          RUSTFLAGS = "-C codegen-backend=cranelift";
          CARGO_PROFILE_DEV_CODEGEN_BACKEND = "cranelift cargo +nightly build -Zcodegen-backend";
        };
        packages = with pkgs; [
          cargo-valgrind
        ];
        languages = {
          shell.enable = true;
          rust = {
            enable = true;
            channel = "nightly";
            components = [
              "rustc"
              "cargo"
              "clippy"
              "rustfmt"
              "rust-analyzer"
              "miri"
            ];
            cranelift.enable = true;
            wild.enable = true;
          };
        };
      };
    };

    release = {
      extends = [
        "production"
      ];
      module = {
        env = {
          SCCACHE_ERROR_LOG = "${config.git.root}/tmp/sccache.log";
          SCCACHE_ENDPOINT = "https://rustc-cache.storm-cdn.com/telepathic";
          SCCACHE_BUCKET = "rustc-cache";
        };
        languages.rust = {
          enable = true;
          channel = "nightly";
          components = [
            "rustc"
            "cargo"
            "rust-std"
          ];
          lld.enable = false;
          cranelift.enable = false;
          wild.enable = false;
        };
      };
    };

    release-unix = {
      extends = [
        "release"
      ];
      module = {
        packages = with pkgs; [
          gcc
          gnumake
          cmake
        ];
      };
    };

    release-cross = {
      extends = [
        "release"
      ];
      module = {
        packages = with pkgs; [
          cargo-zigbuild
        ];
        languages = {
          zig = {
            enable = true;
            lsp.enable = false;
          };
        };
      };
    };

    release-windows = {
      extends = [
        "release"
      ];
      module = {
        packages = with pkgs; [
          cargo-xwin
        ];
        languages.rust = {
          lld.enable = true;
        };
      };
    };

    release-darwin = {
      extends = [
        "release"
      ];
      module = {
        languages.rust = {
          lld.enable = true;
        };
      };
    };

    release-darwin-x86_64 = {
      extends = [
        "release-darwin"
        "release-unix"
      ];
      module = {
        env = {
          NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM = "1";
        };
        languages.rust.targets = [ "x86_64-apple-darwin" ];
      };
    };

    release-linux-musl-x86_64 = {
      extends = [
        "release-cross"
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "x86_64-unknown-linux-musl" ];
      };
    };

    release-linux-gnu-x86_64 = {
      extends = [
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "x86_64-unknown-linux-gnu" ];
      };
    };

    release-linux-gnu-aarch64 = {
      extends = [
        "release-cross"
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "aarch64-unknown-linux-gnu" ];
      };
    };

    release-linux-musl-aarch64 = {
      extends = [
        "release-cross"
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "aarch64-unknown-linux-musl" ];
      };
    };

    release-linux-gnueabihf-armv7 = {
      extends = [
        "release-cross"
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "armv7-unknown-linux-gnueabihf" ];
      };
    };

    release-linux-android-aarch64 = {
      extends = [
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "aarch64-linux-android" ];
      };
    };

    release-linux-android-armv7 = {
      extends = [
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "armv7-linux-androideabi" ];
      };
    };

    release-wasm32-wasip1 = {
      extends = [
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "wasm32-wasip1-threads" ];
      };
    };

    release-windows-x86_64 = {
      extends = [
        "release-windows"
      ];
      module = {
        languages.rust.targets = [ "x86_64-pc-windows-msvc" ];
      };
    };

    release-windows-aarch64 = {
      extends = [
        "release-windows"
      ];
      module = {
        languages.rust.targets = [ "aarch64-pc-windows-msvc" ];
      };
    };

    release-windows-i686 = {
      extends = [
        "release-windows"
      ];
      module = {
        languages.rust.targets = [ "i686-pc-windows-msvc" ];
      };
    };

    release-freebsd-x86_64 = {
      extends = [
        "release-cross"
        "release-unix"
      ];
      module = {
        languages.rust.targets = [ "x86_64-unknown-freebsd" ];
      };
    };
  };
}
