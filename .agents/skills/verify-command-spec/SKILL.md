---
name: verify-command-spec
description: Orchestrate and verify command-spec Oz runs for Linear tickets. Starts an implementation run using the add-command-spec skill, verifies all edited/created generators have meaningful screenshots, and retries with follow-up runs until validation passes. Use this skill whenever you need to implement and fully validate a command-spec Linear ticket end-to-end, or when a user provides a Linear ticket that involves adding or modifying command completions in the command-signatures repo.
---

# verify-command-spec

You are an orchestration agent managing the full lifecycle of a command-spec ticket: implementation, screenshot verification, and follow-up correction runs. You receive a Linear ticket URL or ID and drive it to completion autonomously.

## Prerequisites

- `oz-dev` CLI (authenticated)
- `gh` CLI (authenticated, with access to `warpdotdev/command-signatures`)
- Linear MCP server connected

## Constants

- **Environment ID**: `ENsVydbZbxsWMrgK13L1Fx`
- **Skill**: `warpdotdev/command-signatures:add-command-spec`
- **Max verification cycles**: 4 (initial run + up to 3 follow-ups)
- **Max failure retries per run**: 2 (so up to 3 attempts per run before giving up)
- **Poll interval**: 60 seconds

## Phase 1: Extract ticket info

Use the Linear MCP `get_issue` tool to fetch the ticket. Record:
- The ticket identifier (e.g. `APP-3478`)
- The ticket title and description — you will need these to construct follow-up prompts later.

## Phase 2: Start the implementation run

Spawn a subagent with computer use enabled.
```sh
oz-dev agent run-cloud \
    --computer-use \
    --environment "SVhg783GBFQHk1OfdPfFU9" \
    --skill "warpdotdev/command-signatures:add-command-spec" \
    --prompt "Use the add_command_spec skill to address this linear ticket: <LINEAR_TICKET>. Change the ticket status to 'In Review' when all steps are finished. Screenshots should be included as images in the PR description following the upload process, or posted to the Linear issue and linked to in the GitHub PR description if that's not possible."
```

Parse the run ID from the output (look for `Spawned agent with run ID: <UUID>`).

## Phase 3: Monitor the run

Poll `oz-dev run get <RUN_ID>` every 60 seconds until the run reaches a terminal state.

- **Succeeded**: Proceed to Phase 4.
- **InProgress**: Wait and poll again.
- **Failed**: Inspect the status message in the output.
  - If it contains **"Environment setup failed"**, this is a configuration error. **Stop immediately** and report the error — do not retry.
  - For any other failure, restart the run by repeating Phase 2. You may retry up to 2 additional times (3 total attempts per run). If all attempts fail, stop and report the failure with the last error message.

## Phase 4: Discover the PR and identify generators

### Find the PR

The run output includes an **Artifacts** section with the PR reference, branch name, and link:

```
Artifacts:
  PR: command-signatures #233
    Branch: app-3478/command-spec-kubectl-kubeconfig
    Link: https://github.com/warpdotdev/command-signatures/pull/233
```

If artifacts are missing, fall back to searching:

```sh
gh pr list --repo warpdotdev/command-signatures --state open --json number,title,headRefName,url
```

Match by ticket identifier in the branch name or PR title.

### Identify generators from the diff

Fetch the diff:

```sh
gh pr diff <PR_NUMBER> --repo warpdotdev/command-signatures
```

Look for changes in two locations:
- **Generator source files**: `command-signatures/src/generators/<command>.rs` — each file may define multiple generator functions (e.g. `users_generator`, `services_generator`). A single file change can mean multiple generators need verification.
- **JSON specs**: `json/<command>.json` — look for `generatorName` references to identify which generators are being wired up.

Build a list of every distinct generator that was added or modified. Include generator function names when identifiable from the diff, not just file paths — this precision matters for screenshot matching later.

Also check the PR description for an explicit summary of what was added; the add-command-spec skill often lists generators there.

## Phase 5: Validate screenshots

For each generator, determine whether a valid screenshot exists that proves it works.

### 5a: Collect screenshots from all sources

**GitHub PR description:**
```sh
gh pr view <PR_NUMBER> --repo warpdotdev/command-signatures --json body -q '.body'
```

**GitHub PR comments:**
```sh
gh pr view <PR_NUMBER> --repo warpdotdev/command-signatures --json comments -q '.comments[].body'
```

Extract image URLs from both (markdown `![...](URL)` or `<img src="URL">` tags).

**Linear issue**: Use the Linear MCP `get_issue` tool, then `extract_images` on the description markdown.

**Linear issue comments**: Use the Linear MCP `list_comments` tool, then `extract_images` on each comment body.

### 5b: Analyze each screenshot

For each image URL found:

1. Download it:
   ```sh
   curl -sL "<IMAGE_URL>" -o /tmp/screenshot_N.png
   ```
2. View the downloaded image with `read_files` to trigger vision analysis.
3. Determine:
   - **Which generator(s)** does this screenshot demonstrate? Look for: the command being typed, visible completions dropdown entries, flags or subcommands in the input, filenames or paths that indicate which generator was invoked.
   - **Is the screenshot meaningful?**
     - It must show the completions menu (a dropdown next to the cursor) with real, generator-produced entries.
     - A screenshot that shows the command typed but no completions appearing is **not valid**.
     - A screenshot showing an empty completions menu or only static (non-generator) completions is **not valid**.
     - Each generator needs at least one screenshot showing it producing non-trivial output.

### 5c: Build the verification map

Create a mapping: `generator → valid screenshot(s)`. Any generator with zero valid screenshots is **unverified**.

If all generators are verified, skip to Phase 7.

## Phase 6: Follow-up runs

When generators still lack valid screenshots:

### 6a: Update Linear status

If the ticket was moved to "In Review", move it back to "In Progress" using the Linear MCP `save_issue` tool:

```json
{ "id": "<TICKET_ID>", "state": "In Progress" }
```

### 6b: Construct the follow-up prompt

Read `references/follow-up-prompt-template.md` and fill in the placeholders with:
- The ticket description from Phase 1
- The branch name and PR URL from Phase 4
- The specific list of unverified generators and what each should produce
- Context about what was already verified vs. what's still missing

### 6c: Start the follow-up run

```sh
oz-dev agent run-cloud \
    --computer-use \
    --environment "ENsVydbZbxsWMrgK13L1Fx" \
    --skill "warpdotdev/command-signatures:add-command-spec" \
    --prompt "<CONSTRUCTED_FOLLOW_UP_PROMPT>"
```

### 6d: Monitor and re-validate

Repeat Phases 3–5 for the follow-up run. The same failure retry limits apply per run.

This verification loop may repeat up to 3 more times after the initial cycle (4 total verification cycles). If generators still lack valid screenshots after all cycles, proceed to Phase 7 and report which remain unverified.

## Phase 7: Completion

### All generators verified
- Ensure the Linear ticket is in "In Review" via `save_issue`.
- Report success: the PR URL, the verified generators, and how many cycles it took.

### Some generators unverified after all attempts
- Leave the Linear ticket in "In Progress".
- Report which generators still need screenshots, the PR URL, what succeeded, and any patterns you noticed (e.g. a generator that never produces output may be broken rather than just unscreenshotted).
