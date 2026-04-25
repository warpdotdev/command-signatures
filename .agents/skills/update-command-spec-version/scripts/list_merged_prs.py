#!/usr/bin/env python3
"""List merged PRs in command-signatures between two commit hashes.

Outputs JSON with categorized PRs:
  - new_completions: titles starting with "add" (case-insensitive)
  - bug_fixes: titles containing "fix" or "bug" (case-insensitive)
  - other: everything else

Usage:
    python3 list_merged_prs.py <old_hash> <new_hash>
"""

import json
import os
import re
import subprocess
import sys


REPO_DIR = os.path.expanduser("~/command-signatures")
GH_REPO = "warpdotdev/command-signatures"


def run(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error running {' '.join(cmd)}:\n{result.stderr}", file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip()


def extract_pr_numbers(log_output):
    """Extract unique PR numbers from squash-merge commit messages like '(#123)'."""
    return sorted(set(re.findall(r"\(#(\d+)\)", log_output)), key=int)


def fetch_pr_title(pr_number):
    try:
        return run([
            "gh", "pr", "view", str(pr_number),
            "--repo", GH_REPO,
            "--json", "title",
            "--jq", ".title",
        ])
    except SystemExit:
        return f"(could not fetch title for PR #{pr_number})"


def categorize(title):
    lower = title.lower()
    if lower.startswith("add"):
        return "new_completions"
    if "fix" in lower or "bug" in lower:
        return "bug_fixes"
    return "other"


def main():
    if len(sys.argv) != 3:
        print("Usage: list_merged_prs.py <old_hash> <new_hash>", file=sys.stderr)
        sys.exit(1)

    old_hash, new_hash = sys.argv[1], sys.argv[2]

    log_output = run([
        "git", "-C", REPO_DIR,
        "log", "--oneline", f"{old_hash}..{new_hash}",
    ])

    if not log_output:
        print(json.dumps({
            "prs": [],
            "new_completions": [],
            "bug_fixes": [],
            "other": [],
        }))
        return

    pr_numbers = extract_pr_numbers(log_output)

    if not pr_numbers:
        # No PR numbers found — list raw commits instead so the agent has context.
        commits = []
        for line in log_output.splitlines():
            parts = line.split(" ", 1)
            commits.append({"sha": parts[0], "message": parts[1] if len(parts) > 1 else ""})
        print(json.dumps({
            "prs": [],
            "commits_without_prs": commits,
            "new_completions": [],
            "bug_fixes": [],
            "other": [],
        }, indent=2))
        return

    prs = []
    buckets = {"new_completions": [], "bug_fixes": [], "other": []}

    for num in pr_numbers:
        title = fetch_pr_title(num)
        entry = {"number": int(num), "title": title}
        prs.append(entry)
        buckets[categorize(title)].append(entry)

    result = {"prs": prs, **buckets}
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
