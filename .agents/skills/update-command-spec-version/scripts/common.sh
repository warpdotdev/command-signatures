#!/bin/bash
# Shared repo-path resolution for the update-command-spec-version scripts.
#
#   CMD_SIGS_DIR - the command-signatures repo, inferred from this script's own
#                  location (these scripts live inside that repo), so it always
#                  points at the checkout the skill ships from.
#   WARP_DIR     - the warp repo, taken from the $WARP_DIR environment variable
#                  when set; otherwise a sibling of the command-signatures repo
#                  (same parent directory, named "warp").

# Directory this file lives in (resolved through symlinks).
_COMMON_SH_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# The command-signatures repo root that contains these scripts.
CMD_SIGS_DIR="$(git -C "${_COMMON_SH_DIR}" rev-parse --show-toplevel)"

# The warp repo: prefer $WARP_DIR, else a sibling of command-signatures.
if [ -n "${WARP_DIR:-}" ]; then
    WARP_DIR="${WARP_DIR}"
else
    WARP_DIR="$(dirname "${CMD_SIGS_DIR}")/warp"
fi

export CMD_SIGS_DIR WARP_DIR
