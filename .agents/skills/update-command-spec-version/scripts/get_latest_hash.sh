#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/common.sh"

# After ensure_repos.sh has fetched, origin/main is up to date.
HASH=$(git -C "${CMD_SIGS_DIR}" rev-parse origin/main)

if [ -z "${HASH}" ]; then
    echo "Error: could not determine HEAD of command-signatures main" >&2
    exit 1
fi

echo "${HASH}"
