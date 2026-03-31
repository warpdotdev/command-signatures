---
name: add-command-spec
description: Guide for adding new command completion specs to warp-command-signatures. Use when creating a new JSON spec for shell command completions, adding generators for dynamic suggestions, or extending existing command specs.
---

# Adding a New Command Spec

This skill covers the full lifecycle of writing a completion spec in warpdotdev/command-signatures: researching the command, implementing the spec, validating it, submitting a PR, and documenting learnings for future completion spec implementations.

## Step 1: Research the Command

Before writing any JSON, build a thorough picture of the command's subcommands, flags, and argument types. Commands often have more surface area than you'd expect — nested subcommands, platform-specific flags, mutually exclusive options. Investing time here prevents rework later.

Use these strategies roughly in priority order:

### Start with Fish shell completions

Fish maintains high-quality, community-reviewed completion definitions at https://github.com/fish-shell/fish-shell/tree/master/share/completions — look for `<command>.fish`. These are thorough and well-structured, so they're the fastest way to get a comprehensive picture of a command's subcommands and flags. Read this file first.

### Test with Fish shell completions

You can also use Fish's completion engine to test output interactively:

1. Install Fish if needed (`brew install fish` on UNIX)
2. Run: `fish -c 'complete -C "<command> "'` to see top-level completions
3. Drill into subcommands: `fish -c 'complete -C "<command> <subcommand> "'`

For example, to inspect `gcloud compute ssh` completions: `fish -c 'complete -C "gcloud compute ssh w"'` (where `w` is the start of a known target).

### Install and inspect the command directly

Use the command's own documentation to fill gaps and verify what Fish reports. Install the command if it isn't already available, then:

- Check the `man` page (pipe to `cat` to avoid the pager): `man <command> | cat`
- Run `<command> --help` and `<command> help` at each subcommand level to discover nested structure
- Run the command itself to observe real output — this matters for generators that parse command output

### Use Fig specs as a last resort

The Fig autocomplete repo at https://github.com/withfig/autocomplete/tree/master/src has TypeScript specs for many commands (look for `<command>.ts`). These can help fill gaps, but they vary in quality and may be outdated — always verify against the command's own docs.

## Step 2: Implement the Spec

1. **Create the JSON spec**: `command-signatures/json/<command>.json` following the [Fig completion spec schema](https://fig.io/docs/reference)
2. **Create a generator** (if needed): Add `command-signatures/src/generators/<command>.rs`, define a `generator()` function returning `CommandSignatureGenerators`, and register it in `generators/mod.rs`

## Platform Compatibility

When implementing generator commands, ensure they work across all applicable platforms where the command exists. For example, a UNIX-only command should work on both macOS and Linux, not just the platform being used for development.

### Common pitfalls

- Commands that work differently across platforms (for example, user lookup via `dscl` on macOS vs `getent` on Linux)
- Commands with different output formats across platforms
- Hardcoded paths that differ between systems

### Solutions

Identify which platforms the command needs to support.

Prioritize approaches in this order:

1. **Use cross-platform commands** when available — commands that work identically on the relevant platforms minimize maintenance burden. However, this is not always possible.
2. **Feature detection** — prefer testing for command availability or flag support over platform checks:
   - `command -v <cmd>` to check if a tool exists, see `fn users_generator()` in `command-signatures/src/generators/common.rs` for an example.
   - `<cmd> --version 2>/dev/null` or `<cmd> --help` to test flag support
3. **Graceful fallbacks** — when a platform-specific tool is unavailable, fall back to portable alternatives (e.g., `getent` → `dscl` → `/etc/passwd`).
4. **Platform detection as last resort** — only use `uname` or similar if the above approaches are insufficient.

Implement platform-specific logic in the generator only when behavior fundamentally differs across systems.

## Generator Reusability

Generators that are shared by multiple commands should live in `command-signatures/src/generators/common.rs`. Before implementing a new generator:

1. Search the codebase to see if a similar generator has already been implemented by another command.
2. If one exists and can be reused, use it directly.
3. If a generator is used across multiple commands, move it to `common.rs` for reuse.
4. Generators that are only used by a single command should remain in their own module (e.g., `command-signatures/src/generators/<command>.rs`).

See `fn users_generator()` in `common.rs` as an example of a cross-platform generator used by multiple commands.

## Style Guideline

Match the formatting conventions used in the command's `--help` output. For example, if the help text uses `UPPER_CASE` for positional argument names, use the same casing in the spec's argument `name` field.

### Documenting Argument Formats

If an argument has a specific format (date, time, pattern, etc.), document it in the argument's `description` field. This helps users understand the expected input format.

Example:
```json
"args": {
    "name": "TIME",
    "description": "Time to set (format: \"YYYY-MM-DD HH:MM:SS\")"
}
```

## Validation

Format the JSON spec with `npm run format -- command-signatures/json/<command>.json`.

Run `script/presubmit` to verify formatting, linting, and tests all pass (this runs `cargo fmt --check`, `cargo clippy`, and `cargo test`).

To verify completions end-to-end in a real Warp session, use the **test-local-warp** skill, which covers building and running Warp against a local checkout of this repo. This requires computer use to be enabled since Warp is a GUI application.
Use this when adding generators to make sure they produce reasonable candidates.
Produce screenshots for each generator added so that a human can review them. You do not need to zoom in.
We only need screenshots for generator functions. Things like sub-commands and options are already well tested.
If you are running unsupervised (autonomy level is `UNSUPERVISED`), screenshot artifacts are required; the generator implementation will not be accepted without them. When running supervised, skip this step.

## Submitting

Title the branch: `lucie/test-command-spec-<short-name>`.
Title the PR: **Add completion spec: `<command full name> [short-name])`**.
<command full name> is the command's full, human-readable name (eg. "ripgrep").
[short-name] is the command's CLI invocation, if it exists, in parentheses (eg. "(rg)").
For example, adding support for ripgrep would be done in a branch called `lucie/test-command-spec-rg` and a PR titled "Add completion spec: ripgrep (rg)".

A consistent title convention makes it easy to scan PR history and understand what was added at a glance.

## Reference Examples

- **Simple spec with generator**: `json/kill.json` + `src/generators/kill.rs` — minimal example showing `generatorName` usage for process and signal completions
- **Complex spec with multiple generators**: `json/brew.json` + `src/generators/brew.rs` — shows subcommands, options, and multiple generators (`formulae_generator`, `services`, etc.)
