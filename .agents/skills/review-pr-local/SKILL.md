---
name: review-pr-local
specializes: review-pr
specializes_source: warpdotdev/common-skills:.agents/skills/review-pr
description: Repo-specific review guidance for command-signatures. Only the categories declared overridable by the core review-pr skill may be specialized here.
---

# Repo-specific review guidance for `command-signatures`

## Prerequisite: install the parent skill

This skill specializes the core `review-pr` skill (named in the `specializes` frontmatter field) and is not functional on its own. Before applying its guidance, confirm the parent skill is installed and resolvable at `.agents/skills/review-pr/SKILL.md`. If it is missing, install it first by copying the skill directory from the source declared in the `specializes_source` frontmatter field (`warpdotdev/common-skills:.agents/skills/review-pr`). Then continue with the guidance below.

This file is a companion to the core `review-pr` skill. It does not redefine the review output schema, severity labels, safety rules, or evidence rules. It only specializes the override categories the core skill marks as overridable.

## Implementation standard: read `add-command-spec` first

Before reviewing, read `.agents/skills/add-command-spec/SKILL.md`. That skill defines the authoritative implementation standard for this repo — research process, JSON spec conventions, generator requirements, cross-platform rules, style guidelines, presubmit steps, and branch/PR naming. Use it as your rubric: flag any deviation from what it prescribes. The sections below cover only the review-specific concerns that go beyond that standard.

## Screenshots for generators are mandatory

Every new or modified generator must have at least one screenshot embedded in the PR description proving it produces completions in a real Warp session.

- A valid screenshot shows the completions **dropdown menu** with real, generator-produced entries next to the cursor.
- A screenshot showing the command typed without a visible dropdown, or a dropdown with no entries, is **not valid**.
- Screenshots must be embedded in the PR body as markdown images — not committed as files in the repo.
- If any added or modified generator lacks a valid screenshot, set the verdict to `Request changes` and request one explicitly. This applies even if all other review criteria pass.

To find which generators were added or modified, look for:
- Changes in `command-signatures/src/generators/<command>.rs` — each file may define multiple generator functions.
- `generatorName` references in changed `json/<command>.json` files.

## Generator registration consistency

Every `generatorName` value in a JSON spec must have a corresponding entry in `generators/mod.rs` via `dynamic_command_signature_data()`. Flag any `generatorName` not present in `mod.rs`, or any generator registered in `mod.rs` that is never referenced by a spec.

## Validate spec correctness against authoritative sources

Do not trust that the submitted spec is accurate — actively verify it against at least one authoritative source, using the same priority order defined in `add-command-spec`:

1. **Fish shell completions** at `https://github.com/fish-shell/fish-shell/tree/master/share/completions/<command>.fish` — check related generator functions in `share/functions/` too.
2. **`--help` output** — run `<command> --help` and `<command> <subcommand> --help` if the command is available.
3. **Fig autocomplete repo** at `https://github.com/withfig/autocomplete/tree/master/src/<command>.ts` — lower confidence, verify against other sources.
4. **Official docs / man page** — fallback when the above are insufficient.

When verifying:
- Flag subcommands or options present in the authoritative source but absent from the spec without explanation.
- Flag options in the spec that do not appear in any authoritative source (possible typos, stale flags, or hallucinated entries).
- Flag descriptions that are factually wrong or misleading.
- Minor omissions (a rarely-used flag left out of an otherwise complete spec) are low-severity suggestions. Focus blocking comments on missing top-level subcommands, wrong argument cardinality, or incorrect flag names.

## Presubmit

If CI output is visible, verify `script/presubmit` is green (it runs `cargo fmt --check`, `cargo clippy`, and `cargo test`). Flag open failures.
