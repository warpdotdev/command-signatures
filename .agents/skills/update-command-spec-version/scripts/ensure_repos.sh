#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/common.sh"

# command-signatures is the repo these scripts live in, so it always exists.
# Only refresh its remotes; never mutate the working tree or checked-out branch
# (the skill reads origin/main directly via get_latest_hash.sh).
echo "Fetching command-signatures (${CMD_SIGS_DIR})..." >&2
git -C "${CMD_SIGS_DIR}" fetch origin

if [ -d "${WARP_DIR}" ]; then
    echo "Fetching warp (${WARP_DIR})..." >&2
    git -C "${WARP_DIR}" fetch origin
else
    echo "Cloning warp into ${WARP_DIR}..." >&2
    git clone ssh://git@github.com/warpdotdev/warp.git "${WARP_DIR}"
fi

echo "Both repositories are ready." >&2
