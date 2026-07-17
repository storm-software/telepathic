{
  pkgs,
  inputs,
  config,
  lib,
  ...
}:
let
  # Prebuilt wasi-sdk sysroot for compiling C deps (tree-sitter) to wasm32-wasip1-threads.
  # nixpkgs.wasilibc is WASI-only and does not build cleanly as a host package.
  wasi-sdk-sysroot = pkgs.stdenv.mkDerivation {
    pname = "wasi-sdk-sysroot";
    version = "27.0";
    src = pkgs.fetchurl {
      url = "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-27/wasi-sdk-27.0-x86_64-linux.tar.gz";
      hash = "sha256-t9TZRMiFA+TyHYSvB6wpPjRAsbYhC/1/544K/ZLCO8I=";
    };
    sourceRoot = "wasi-sdk-27.0-x86_64-linux";
    dontConfigure = true;
    dontBuild = true;
    # Sysroot is wasm bitcode/headers only; skip host ELF fixup.
    dontFixup = true;
    installPhase = ''
      mkdir -p $out
      cp -a share/wasi-sysroot $out/sysroot
    '';
  };
in
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

  packages =
    with pkgs;
    [
      sccache
      libclang
      pkg-config
      openssl
    ]
    ++ lib.optionals pkgs.stdenv.isLinux [
      # Tauri desktop (Linux) — webkitgtk is broken/unsupported on Darwin
      webkitgtk_4_1
      gtk3
      librsvg
      gdk-pixbuf
      cairo
      pango
      linuxdeploy
    ];

  env = {
    # bindgen loads libclang at build time; point it at nix libclang, not host /usr/lib.
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
    SCCACHE_ENDPOINT = "https://d011605e7391539ac2e021ab4399e116.r2.cloudflarestorage.com";
    SCCACHE_BUCKET = "telepathic-rustc-cache";
    SCCACHE_REGION = "auto";
    SCCACHE_ERROR_LOG = "${config.git.root}/tmp/sccache.log";
  }
  // lib.optionalAttrs pkgs.stdenv.isLinux {
    # linuxdeploy (AppImage) resolves GTK/WebKit via ldd; Nix libs are not on default search path.
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.webkitgtk_4_1
      pkgs.gtk3
      pkgs.glib
      pkgs.librsvg
      pkgs.gdk-pixbuf
      pkgs.cairo
      pkgs.pango
    ];
  };

  languages.c.enable = true;

  tasks = {
    "telepathic:setup:tmp" = {
      exec = ''
        mkdir -p "${config.git.root}/tmp"
      '';
      before = [
        "devenv:enterShell"
        "devenv:enterTest"
      ];
      after = [
        "devenv:files"
        "devenv:files:cleanup"
      ];
    };
  };

  scripts = {
    build-native.exec = "${config.git.root}/tools/scripts/src/build-native.sh \"$@\"";
    codegen-lang.exec = "pnpm codegen-lang";
    new-lang.exec = "pnpm new-lang $1 $2";
    desktop-dev.exec = "pnpm --filter @telepathic/desktop tauri:dev";
    desktop-build.exec = "pnpm --filter @telepathic/desktop tauri:build";
  };

  profiles = {
    debug = {
      extends = [
        "development"
      ];
      module = {
        env = {
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
          linuxdeploy
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
        packages =
          with pkgs;
          [
            cargo-xwin
            gcc
          ]
          ++ lib.optionals pkgs.stdenv.isLinux [ glibc.static ];
        # Host build scripts must use gcc, not clang: lld.enable makes rustc link
        # host bins with clang → __tls_get_addr / DSO missing. cargo-xwin owns
        # Windows target linking.
        #
        # Do NOT set NIX_LDFLAGS to glibc/glibc.static: Nix cc wrapper bakes those
        # -L paths into host RPATH and host build-script bins SIGSEGV (proc-macro2,
        # quote, serde_core).
        #
        # Windows +crt-static still leaks into CARGO_CFG_TARGET_FEATURE for build
        # scripts; cc-rs may pass -static when compiling host tools (rlemon).
        # LIBRARY_PATH (link-time only, dynamic first) covers libc.a without
        # poisoning host RPATH. Clear desktop GTK LD_LIBRARY_PATH (not needed).
        #
        # .cargo/config.toml sets target-applies-to-host = false so +crt-static
        # rustflags stay off host artifacts.
        env = lib.optionalAttrs pkgs.stdenv.isLinux {
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.stdenv.cc}/bin/cc";
          LIBRARY_PATH = "${pkgs.glibc}/lib:${pkgs.glibc.static}/lib";
          # Override desktop GTK LD_LIBRARY_PATH from top-level env.
          LD_LIBRARY_PATH = lib.mkForce "";
        };
        languages.rust = {
          lld.enable = false;
        };
      };
    };

    release-darwin = {
      extends = [
        "release"
      ];
      module = {
        languages = {
          rust = {
            lld.enable = true;
          };
          python = {
            manylinux.enable = lib.mkForce false;
          };
        };
      };
    };

    release-darwin-x86_64 = {
      extends = [
        "release-darwin"
      ];
      module = {
        env = {
          NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM = "1";
          # nix cc-wrapper is host-only (arm64 on macos-latest). Apple clang is
          # multi-arch — required when cross-compiling x86_64-apple-darwin C deps
          # (tree-sitter / lsp). Without this: "x86_64-apple-macosx != arm64-apple-darwin".
          CC_x86_64_apple_darwin = "/usr/bin/clang";
          CXX_x86_64_apple_darwin = "/usr/bin/clang++";
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

    # Shared Android cross profile. napi-rs prepends NDK clang to PATH and sets
    # TARGET_CC; without pinning the host linker, build scripts can end up on
    # host glibc. bindgen then dlopens nix libclang → nix libdl (no RUNPATH)
    # binds to already-loaded host libc → GLIBC_ABI_DT_X86_64_PLT missing.
    #
    # Do NOT put pkgs.glibc in LD_LIBRARY_PATH: that segfaults host git/node
    # during devenv enterShell (storm:setup:git, pnpm bootstrap).
    release-linux-android = {
      extends = [
        "release-unix"
      ];
      module = {
        env = lib.optionalAttrs pkgs.stdenv.isLinux {
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.stdenv.cc}/bin/cc";
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.stdenv.cc}/bin/cc";
          # libclang + llvm libs for bindgen dlopen. Overrides desktop GTK
          # LD_LIBRARY_PATH (not needed for Android builds). No glibc here.
          LD_LIBRARY_PATH = lib.makeLibraryPath [
            pkgs.libclang.lib
            pkgs.llvmPackages.llvm.lib
            pkgs.stdenv.cc.cc.lib
          ];
        };
      };
    };

    release-linux-android-aarch64 = {
      extends = [
        "release-linux-android"
      ];
      module = {
        languages.rust.targets = [ "aarch64-linux-android" ];
      };
    };

    release-linux-android-armv7 = {
      extends = [
        "release-linux-android"
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
        env = {
          # cc-wrapper injects host hardening flags (e.g. -fzero-call-used-regs=used-gpr)
          # that wasm clang rejects when building native deps like tree-sitter.
          NIX_HARDENING_ENABLE = "";
          # cc-rs needs WASI_SYSROOT for C deps (tree-sitter); bare clang has no stdio.h.
          WASI_SYSROOT = "${wasi-sdk-sysroot}/sysroot";
          CC_wasm32_wasip1_threads = "${pkgs.llvmPackages.clang-unwrapped}/bin/clang";
          CXX_wasm32_wasip1_threads = "${pkgs.llvmPackages.clang-unwrapped}/bin/clang++";
        };
        packages = with pkgs; [
          llvmPackages.clang-unwrapped
          wasi-sdk-sysroot
        ];
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
