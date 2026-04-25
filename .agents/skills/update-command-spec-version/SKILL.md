---
name: update-command-spec-version
description: >
  Update the warp-command-signatures dependency hash in warp-internal to the latest commit from warpdotdev/command-signatures:main.
---

# Update Command Spec Version

This skill automates updating the `warp-command-signatures` git dependency in `warpdotdev/warp-internal` to the latest commit on `warpdotdev/command-signatures:main`, then opens a PR with a summary of what changed.

All scripts live in this skill's `scripts/` directory.

## Prerequisites

- `gh` CLI authenticated with access to `warpdotdev/warp-internal` and `warpdotdev/command-signatures`
- SSH access to both repos (they use `ssh://git@github.com/warpdotdev/...`)
- Python 3 (for `list_merged_prs.py`)
- Rust toolchain with `cargo` (for updating `Cargo.lock`)

## Step-by-step procedure

Run the scripts in order. The skill directory is wherever this SKILL.md lives; reference scripts relative to it.

### 1. Ensure repos are available

```bash
bash <skill-dir>/scripts/ensure_repos.sh
```

This clones or fetches both `~/warp-internal` and `~/command-signatures`.

### 2. Get the current and latest hashes

```bash
OLD_HASH=$(bash <skill-dir>/scripts/get_current_hash.sh)
NEW_HASH=$(bash <skill-dir>/scripts/get_latest_hash.sh)
```

If `OLD_HASH` equals `NEW_HASH`, tell the user that command-signatures is already up to date and stop.

### 3. List merged PRs

```bash
python3 <skill-dir>/scripts/list_merged_prs.py "$OLD_HASH" "$NEW_HASH"
```

This outputs JSON with four keys: `prs` (all PRs), `new_completions`, `bug_fixes`, and `other`. Each entry has `number` and `title`.

Review the categorization. The script uses simple heuristics (titles starting with "add" → new completion, titles containing "fix" or "bug" → bug fix). Move any miscategorized entries before composing the description.

### 4. Create the branch and update dependencies

```bash
bash <skill-dir>/scripts/update_and_branch.sh "$NEW_HASH"
```

This checks out a new branch `completions-bot/update-command-signatures-<first 8 chars of NEW_HASH>` from `origin/master` in `~/warp-internal`, updates the `rev` in `Cargo.toml`, and syncs `Cargo.lock`.

### 5. Compose the PR description

Write a PR description following this structure (match the warp-internal PR template). Save it to a temp file, e.g. `/tmp/completions_pr_body.md`.

```
## Description

Updates `warp-command-signatures` to <NEW_HASH short>.

### Merged PRs
- <PR title> (warpdotdev/command-signatures#<number>)
- <PR title> (warpdotdev/command-signatures#<number>)
- ...

## Changelog Entries for Stable

CHANGELOG-IMPROVEMENT: <single line covering all new completions, subcommands, generators, and updates>
CHANGELOG-BUG-FIX: <single line covering all bug fixes>
```

Guidelines for changelog entries:
- Write **one** `CHANGELOG-IMPROVEMENT` line that summarizes all new completions and improvements. If there are many commands, use only their short names. For example: "Added completions for `aws ec2`, `scp`, `claude`, and `docker-compose` service names."
- Write **one** `CHANGELOG-BUG-FIX` line that summarizes all bug fixes. For example: "Fixed npm install short-form command priority and HTML entity rendering in completion specs."
- Omit a changelog line entirely if there are no PRs in that category.
- If a PR doesn't fit either category, use your judgment — it may belong in the improvement line or may not warrant a changelog entry at all.

### 6. Submit the PR

```bash
bash <skill-dir>/scripts/submit_pr.sh "$NEW_HASH" <absolute_path_to_temp_description_file>
```

This commits the changes, pushes the branch, and opens a draft PR titled "[Completions] Bump command-signatures to <first 8 chars of NEW_HASH>" against `master`.
