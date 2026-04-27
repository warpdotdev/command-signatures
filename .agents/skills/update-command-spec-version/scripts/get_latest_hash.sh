#!/bin/bash
set -euo pipefail

CMD_SIGS_DIR="${HOME}/command-signatures"

# After ensure_repos.sh has pulled, origin/main is up to date.
HASH=$(git -C "${CMD_SIGS_DIR}" rev-parse origin/main)

if [ -z "${HASH}" ]; then
    echo "Error: could not determine HEAD of command-signatures main" >&2
    exit 1
fi

echo "${HASH}"
