#!/bin/bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: update_and_branch.sh <new_hash>" >&2
    exit 1
fi

NEW_HASH="$1"
WARP_DIR="${HOME}/warp"
CARGO_TOML="${WARP_DIR}/Cargo.toml"
BRANCH_NAME="completions-bot/update-command-signatures-${NEW_HASH:0:8}"

cd "${WARP_DIR}"

# Create a fresh branch from origin/master.
git checkout -B "${BRANCH_NAME}" origin/master

# Find the current rev on this branch's Cargo.toml.
CURRENT_HASH=$(grep 'warp-command-signatures.*rev' "${CARGO_TOML}" \
    | grep -o 'rev = "[^"]*"' \
    | grep -o '"[^"]*"' \
    | tr -d '"')

if [ -z "${CURRENT_HASH}" ]; then
    echo "Error: could not find warp-command-signatures rev in Cargo.toml" >&2
    exit 1
fi

if [ "${CURRENT_HASH}" = "${NEW_HASH}" ]; then
    echo "Cargo.toml already points to ${NEW_HASH}. Nothing to update." >&2
    exit 0
fi

echo "Updating rev: ${CURRENT_HASH} → ${NEW_HASH}" >&2

# Replace the rev value in the warp-command-signatures dependency line.
sed -i '' "s|rev = \"${CURRENT_HASH}\"|rev = \"${NEW_HASH}\"|" "${CARGO_TOML}"

echo "Running cargo check to ensure Cargo.lock is in sync..." >&2
cargo check

echo "Branch '${BRANCH_NAME}' is ready with updated Cargo.toml and Cargo.lock." >&2
