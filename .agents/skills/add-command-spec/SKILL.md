---
name: add-command-spec
description: Guide for adding new command completion specs to warp-command-signatures. Use when creating a new JSON spec for shell command completions, adding generators for dynamic suggestions, or extending existing command specs.
---

# Adding a New Command Spec

## Steps

1. **Create JSON spec**: `command-signatures/json/<command>.json` following the [Fig completion spec schema](https://fig.io/docs/reference)
2. **Create generator** (if needed): Add `command-signatures/src/generators/<command>.rs`, define a `generator()` function returning `CommandSignatureGenerators`, and register it in `generators/mod.rs`

## Style Guideline

Match the formatting conventions used in the command's `--help` output. For example, if the help text uses `UPPER_CASE` for positional argument names, use the same casing in the spec's argument `name` field.

## Validation

Run `cargo test` to verify the spec deserializes correctly and any referenced `generatorName`s exist.

## Reference Examples

- **Simple spec with generator**: `json/kill.json` + `src/generators/kill.rs` — minimal example showing `generatorName` usage for process and signal completions
- **Complex spec with multiple generators**: `json/brew.json` + `src/generators/brew.rs` — shows subcommands, options, and multiple generators (`formulae_generator`, `services`, etc.)
