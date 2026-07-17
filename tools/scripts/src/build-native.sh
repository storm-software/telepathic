#!/usr/bin/env bash
# -------------------------------------------------------------------
#
#                   🗲 Storm Software - Telepathic
#
#  This code was released as part of the Telepathic project. Telepathic
#  is maintained by Storm Software under the Apache-2.0 license, and is
#  free for commercial and private use. For more information, please visit
#  our licensing page at https://stormsoftware.com/licenses/projects/telepathic.
#
#  Website:                  https://stormsoftware.com
#  Repository:               https://github.com/storm-software/telepathic
#  Documentation:            https://docs.stormsoftware.com/projects/telepathic
#  Contact:                  https://stormsoftware.com/contact
#
#  SPDX-License-Identifier:  Apache-2.0
#
# -------------------------------------------------------------------

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

normalize_target() {
  local value="$1"
  while [[ "$value" == --target=* ]]; do
    value="${value#--target=}"
  done
  if [[ "$value" == --target ]]; then
    value=""
  fi
  printf '%s' "$value"
}

target="$(normalize_target "${NATIVE_TARGET:-}")"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      if [[ $# -lt 2 || -z "$2" ]]; then
        printf '\033[31m--target requires a value\033[0m\n' >&2
        exit 1
      fi
      target="$(normalize_target "$2")"
      shift 2
      ;;
    --target=*)
      target="$(normalize_target "${1#--target=}")"
      shift
      ;;
    -*)
      printf '\033[31mUnknown option: %s\033[0m\n' "$1" >&2
      exit 1
      ;;
    *)
      if [[ -z "$target" ]]; then
        target="$(normalize_target "$1")"
      fi
      shift
      ;;
  esac
done

if [[ -z "$target" ]]; then
  printf '\033[31mNo build target specified. Pass a target triple or set NATIVE_TARGET.\033[0m\n' >&2
  exit 1
fi

command="pnpm nx run bindings-npm:build-$target"

printf '\033[1;37m ⚙️  Bootstrapping the monorepo before building native %s artifacts...\033[0m\n' "$target"

cd "$REPO_ROOT"

# Share one Cargo target dir with `pnpm build` / `.cargo/config.toml` so
# telepathic-tree-sitter (and other crates) are not recompiled per target.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$REPO_ROOT/dist/target}"
export CARGO_BUILD_TARGET_DIR="${CARGO_BUILD_TARGET_DIR:-$CARGO_TARGET_DIR}"

# Windows MSVC cross (cargo-xwin on Linux): host build-script bins SIGSEGV when
# GTK/Nix LD_LIBRARY_PATH, NIX_LDFLAGS RPATH (esp. glibc.static -L), sccache, or
# clang-as-host-linker leak into the host link/runtime. Strip / override those
# before cargo runs.
#
# Also: cargo-xwin sets clang-cl CFLAGS with `/imsvc`, but `ring` forces the GNU
# `clang` driver on aarch64-pc-windows-msvc. GNU clang treats `/imsvc` as a path
# and rejects nix `-fPIC`. Shim `clang` to rewrite `/imsvc` → `-isystem` and call
# unwrapped clang (see devenv release-windows). Do not shadow `clang-cl`.
case "$target" in
  *-pc-windows-msvc)
    unset LD_LIBRARY_PATH
    unset NIX_LDFLAGS
    unset NIX_LDFLAGS_FOR_BUILD
    unset RUSTC_WRAPPER
    unset RUSTFLAGS
    unset RUSTDOCFLAGS
    # Host units compile for the host triple — pin gcc, not clang.
    if command -v gcc > /dev/null 2>&1; then
      export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$(command -v gcc)"
      export CC="$(command -v gcc)"
    elif command -v cc > /dev/null 2>&1; then
      export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$(command -v cc)"
      export CC="$(command -v cc)"
    fi
    if command -v g++ > /dev/null 2>&1; then
      export CXX="$(command -v g++)"
    elif command -v c++ > /dev/null 2>&1; then
      export CXX="$(command -v c++)"
    fi
    # Avoid nix cc-wrapper injecting -fPIC / hardening into Windows target C.
    export NIX_HARDENING_ENABLE=""

    shim_dir="${TMPDIR:-/tmp}/telepathic-xwin-clang-shim-$$"
    mkdir -p "$shim_dir"
    cat >"$shim_dir/clang" <<'SHIM'
#!/bin/sh
# Prefer devenv-provided unwrapped clang; else first real clang on PATH.
if [ -n "${TELEPATHIC_XWIN_CLANG:-}" ] && [ -x "${TELEPATHIC_XWIN_CLANG}" ]; then
  real="${TELEPATHIC_XWIN_CLANG}"
else
  self_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
  real=""
  IFS=:
  for d in $PATH; do
    [ "$d" = "$self_dir" ] && continue
    if [ -x "$d/clang" ]; then
      real="$d/clang"
      break
    fi
  done
  unset IFS
fi
[ -n "$real" ] || {
  echo "telepathic-xwin-clang-shim: real clang not found (set TELEPATHIC_XWIN_CLANG)" >&2
  exit 127
}
# Rebuild argv: /imsvc (clang-cl) → -isystem (GNU); drop -fPIC for windows-msvc.
n=0
windows_msvc=0
for a in "$@"; do
  case "$a" in
    *windows-msvc*) windows_msvc=1 ;;
  esac
done
for a in "$@"; do
  [ "$a" = "/imsvc" ] && a="-isystem"
  if [ "$windows_msvc" -eq 1 ] && [ "$a" = "-fPIC" ]; then
    continue
  fi
  eval "arg_$n=\$a"
  n=$((n + 1))
done
i=0
set --
while [ "$i" -lt "$n" ]; do
  eval "v=\$arg_$i"
  set -- "$@" "$v"
  i=$((i + 1))
done
exec "$real" "$@"
SHIM
    chmod +x "$shim_dir/clang"
    export PATH="$shim_dir:$PATH"
    ;;
esac

if ! pnpm bootstrap; then
  printf '\033[31mAn error occurred while bootstrapping the monorepo\033[0m\n' >&2
  exit 1
fi

printf '\033[1;37m 🏗️  Building the Telepathic native %s artifacts - running command: \n%s\n\033[0m\n' "$target" "$command"

build_timeout="${NATIVE_BUILD_TIMEOUT:-45m}"

set +e
if command -v timeout > /dev/null 2>&1; then
  # shellcheck disable=SC2086
  timeout "$build_timeout" $command
  exit_code=$?
else
  # shellcheck disable=SC2086
  $command
  exit_code=$?
fi
set -e

if [[ $exit_code -eq 124 ]]; then
  printf '\033[31mBuild timed out after %s while building the Telepathic native %s artifacts\033[0m\n' "$build_timeout" "$target" >&2
  exit 1
fi

if [[ $exit_code -ne 0 ]]; then
  printf '\033[31mAn error occurred while building the Telepathic native %s artifacts\033[0m\n' "$target" >&2
  exit 1
fi

printf '\033[32m ✔ Successfully built the Telepathic native %s artifacts!\033[0m\n\n' "$target"
