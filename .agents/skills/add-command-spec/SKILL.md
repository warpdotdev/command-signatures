---
name: add-command-spec
description: Guide for adding new command completion specs to warp-command-signatures. Use when creating a new JSON spec for shell command completions, adding generators for dynamic suggestions, or extending existing command specs.
---

# Adding a New Command Spec

## Steps

1. **Create JSON spec**: `command-signatures/json/<command>.json` following the [Fig completion spec schema](https://fig.io/docs/reference)
2. **Create generator** (if needed): Add `command-signatures/src/generators/<command>.rs`, define a `generator()` function returning `CommandSignatureGenerators`, and register it in `generators/mod.rs`

## Platform Compatibility

When implementing generator commands, ensure they work across all applicable platforms where the command exists. For example, a UNIX-only command should work on both macOS and Linux, not just the platform being used for development.

### Common pitfalls

- Commands that work differently across platforms (for example, user lookup via `dscl` on macOS vs `getent` on Linux)
- Commands with different output formats across platforms
- Hardcoded paths that differ between systems

### Solutions

- Identify which platforms the command supports
- Use platform detection (for example, `uname`) to handle cross-platform differences
- Implement platform-specific logic in the generator when behavior differs across systems

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

## Reference Examples

- **Simple spec with generator**: `json/kill.json` + `src/generators/kill.rs` — minimal example showing `generatorName` usage for process and signal completions
- **Complex spec with multiple generators**: `json/brew.json` + `src/generators/brew.rs` — shows subcommands, options, and multiple generators (`formulae_generator`, `services`, etc.)
