#!/bin/bash
set -euo pipefail

WARP_INTERNAL_DIR="${HOME}/warp-internal"

# Read Cargo.toml from origin/master so we don't depend on local checkout state.
HASH=$(git -C "${WARP_INTERNAL_DIR}" show origin/master:Cargo.toml \
    | grep 'warp-command-signatures.*rev' \
    | grep -o 'rev = "[^"]*"' \
    | grep -o '"[^"]*"' \
    | tr -d '"')

if [ -z "${HASH}" ]; then
    echo "Error: could not find warp-command-signatures rev in Cargo.toml" >&2
    exit 1
fi

echo "${HASH}"
