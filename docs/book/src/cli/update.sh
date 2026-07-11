#!/usr/bin/env bash
set -eo pipefail

BOOK_ROOT="$(dirname "$(dirname "$0")")"
TELEPATHIC=${1:-"$(dirname "$BOOK_ROOT")/target/debug/telepathic"}

cmd=(
  "$(dirname "$0")/help.py"
  --root-dir "$BOOK_ROOT/"
  --root-indentation 2
  --root-summary
  --out-dir "$BOOK_ROOT/cli/"
  "$TELEPATHIC"
)
echo "Running: $" "${cmd[*]}"
"${cmd[@]}"
