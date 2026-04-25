#!/bin/bash
set -euo pipefail

WARP_INTERNAL_DIR="${HOME}/warp-internal"
CMD_SIGS_DIR="${HOME}/command-signatures"

if [ -d "${WARP_INTERNAL_DIR}" ]; then
    echo "Fetching warp-internal..." >&2
    git -C "${WARP_INTERNAL_DIR}" fetch origin
else
    echo "Cloning warp-internal..." >&2
    git clone ssh://git@github.com/warpdotdev/warp-internal.git "${WARP_INTERNAL_DIR}"
fi

if [ -d "${CMD_SIGS_DIR}" ]; then
    echo "Fetching command-signatures..." >&2
    git -C "${CMD_SIGS_DIR}" fetch origin
    git -C "${CMD_SIGS_DIR}" checkout main --quiet
    git -C "${CMD_SIGS_DIR}" pull origin main --quiet
else
    echo "Cloning command-signatures..." >&2
    git clone ssh://git@github.com/warpdotdev/command-signatures.git "${CMD_SIGS_DIR}"
fi

echo "Both repositories are ready." >&2
