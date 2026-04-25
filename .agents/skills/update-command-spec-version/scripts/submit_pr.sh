#!/bin/bash
set -euo pipefail

if [ $# -ne 2 ]; then
    echo "Usage: submit_pr.sh <new_hash> <pr_body_file>" >&2
    exit 1
fi

NEW_HASH="$1"
PR_BODY_FILE="$2"
WARP_INTERNAL_DIR="${HOME}/warp-internal"
SHORT_HASH="${NEW_HASH:0:8}"
BRANCH_NAME="completions-bot/update-command-signatures-${SHORT_HASH}"
PR_TITLE="[Completions] Bump command-signatures to ${SHORT_HASH}"

if [ ! -f "${PR_BODY_FILE}" ]; then
    echo "Error: PR body file not found: ${PR_BODY_FILE}" >&2
    exit 1
fi

cd "${WARP_INTERNAL_DIR}"

# Verify we're on the expected branch.
CURRENT_BRANCH=$(git branch --show-current)
if [ "${CURRENT_BRANCH}" != "${BRANCH_NAME}" ]; then
    echo "Error: expected branch '${BRANCH_NAME}', but on '${CURRENT_BRANCH}'" >&2
    exit 1
fi

# Stage and commit.
git add Cargo.toml Cargo.lock
git commit -m "${PR_TITLE}

Co-Authored-By: Oz <oz-agent@warp.dev>"

# Push.
git push --set-upstream origin "${BRANCH_NAME}" --no-verify

# Open the PR.
gh pr create \
    --title "${PR_TITLE}" \
    --body-file "${PR_BODY_FILE}" \
    --base master
