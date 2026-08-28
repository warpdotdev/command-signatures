# Tech Spec: Document How to Validate Signature Changes

Companion to `specs/GH367/product.md`. This document is the concrete
implementation contract: exact files, headings, and content sources.

## Files to create/modify

1. **Create** `CONTRIBUTING.md` at the repository root
   (`/CONTRIBUTING.md`, sibling to `README.md` and `AGENTS.md`).
2. **Modify** `README.md` to add a short pointer to `CONTRIBUTING.md`. Insert
   a new `## Contributing` section directly above the existing `## License`
   section (currently `README.md:62`), containing one or two sentences
   pointing to `CONTRIBUTING.md` for validation/testing steps. Do not move or
   rewrite any existing `README.md` content.

If the maintainer redirects file placement per open question 1 in
`product.md`, fold the same section content into `README.md` under a new
`## Contributing` heading instead of creating `CONTRIBUTING.md`, and skip the
`README.md` pointer edit (it becomes unnecessary).

## `CONTRIBUTING.md` structure

Use exactly these top-level sections, in this order. Each section's content
requirement is grounded in a specific existing file — cite it in the prose so
future edits know what to keep in sync.

### `# Contributing to command-signatures`
One or two sentences of framing: this repo provides Warp's command completion
specs, and this doc explains how a change is validated before merge. May
link to `README.md` for what specs/generators are and to `AGENTS.md` for
architecture, without duplicating either.

### `## Before You Open a PR`
Source: `script/presubmit` (repo root, 15 lines).
Content requirements:
- Instruct running `script/presubmit` from the repo root before opening a
  PR.
- List, in order, the four checks it runs, with their literal commands, taken
  verbatim from `script/presubmit`:
  1. `npm run format:check` — checks JSON formatting via Prettier (see
     `package.json`'s `format:check` script and `.prettierrc.json`).
  2. `cargo fmt -p warp-command-signatures -p warp-completion-metadata
     --check` — Rust formatting.
  3. `cargo clippy -p warp-command-signatures -p warp-completion-metadata
     --all-targets --all-features -- -D warnings` — lint, warnings treated as
     errors.
  4. `cargo test --verbose` — the Rust test suite (see "Testing Invariants"
     in `AGENTS.md` for what these tests assert about specs).
- Note the two toolchains needed to run it: Node (for the JSON formatter,
  installed via `npm ci`) and the Rust toolchain pinned in
  `rust-toolchain.toml`.
- Do not claim `script/presubmit` autofixes anything — both the JSON format
  check and `cargo fmt --check` are check-only; mention `npm run format` (no
  `:check` suffix) and plain `cargo fmt` as the corresponding commands a
  contributor can run to actually fix formatting before re-running
  `script/presubmit`.

### `## Continuous Integration`
Source: `.github/workflows/CI.yml` (37 lines).
Content requirements:
- State plainly that every PR triggers three CI jobs, and that they check the
  same four things `script/presubmit` checks locally, split as: `format` job
  → `npm run format:check`; `lint` job → `cargo fmt --check` and `cargo
  clippy -D warnings`; `test` job → `cargo test --verbose`.
- State that passing `script/presubmit` locally is a strong predictor of
  passing CI, since the two run the same commands.
- Do not invent any CI behavior beyond what's in `CI.yml` (e.g. do not
  mention coverage thresholds, release jobs, or a CLA-bot check — none exist
  in `.github/workflows/`).

### `## Testing Generators Against a Local Warp Build`
Source: `.agents/skills/test-local-warp/SKILL.md`, adapted for a human
contributor (not a verbatim copy, and not a reference to the skill file,
since a human contributor has no reason to have `.agents/skills/` open).
Content requirements — state explicitly, near the top of this section, that
**generator-backed (dynamic) completions are not covered by CI or `cargo
test`** (per "Testing Invariants" in `AGENTS.md`, the automated tests only
check that spec JSON deserializes and that referenced generator names exist
— they don't check a generator's actual output), so this manual procedure is
the only way to verify one. Then walk through, as ordered steps written for
a human:
1. Prerequisite: a local checkout of both `warpdotdev/warp` and this repo,
   side by side.
2. In the `warp` repo's root `Cargo.toml`, find the `warp-command-signatures`
   entry under `[workspace.dependencies]` and temporarily replace it with a
   path dependency pointing at this repo's `command-signatures/`
   subdirectory (note the nested `command-signatures/command-signatures`
   path — the outer directory is this repo's root, the inner one is the
   crate).
3. From the `warp` repo, run `cargo run --features fast_dev` to build and
   launch a local Warp build.
4. In that running Warp, type the command you changed and press `Tab` to
   open the completions menu (not autocomplete/ghost text) and confirm the
   new/changed spec or generator produces reasonable suggestions. Call out
   the caveat that the completions menu does not work with equal-sign
   flags (`--foo=bar`) — use space-delimited syntax (`--foo bar`) when
   testing.
5. Revert the `Cargo.toml` change in the `warp` repo before committing
   anything — the local path override must never be checked in to `warp`.
- Do not reference computer-use tooling, Warp agent skills, or
  `.agents/skills/add-command-spec/SKILL.md`'s screenshot-upload workflow —
  those are internal-agent submission requirements, not something a human
  contributor's PR is held to by this document. If maintainers want human PRs
  to include a screenshot too, that is a separate decision not requested by
  this issue; do not add it silently.

### `## License and CLA` (or fold into existing `## License` cross-reference)
One sentence pointing back to `README.md`'s existing "License" section
(`README.md:62-67`) for the MIT license and CLA requirement
(https://cla.warp.dev). Do not restate the CLA legal language here — see
`product.md` open question 3 if the maintainer wants it duplicated instead.

## Non-goals / edge cases to watch for during implementation
- Do not add a markdown linter or any new tooling; there is none in this
  repo today (`package.json`'s only scripts are `format`/`format:check`).
- Do not add a `CONTRIBUTING.md` reference to `.github/CONTRIBUTING.md`
  instead of the root — GitHub recognizes root, `.github/`, or `docs/`
  locations, but the root is simplest and matches where `README.md`,
  `AGENTS.md`, and `LICENSE.md` already live in this repo.
- Keep the new doc's four `script/presubmit` commands byte-for-byte
  consistent with `script/presubmit`'s actual content at implementation
  time — re-read the file rather than trusting this spec's quoted text, in
  case it changes between spec approval and implementation.
- `AGENTS.md` and the new `CONTRIBUTING.md` will now both describe
  `script/presubmit`/CI to some degree (`AGENTS.md`'s "Testing Invariants"
  section describes what the tests assert, not how to run them). This is
  acceptable overlap, not duplication to eliminate — `AGENTS.md` remains
  agent-oriented, `CONTRIBUTING.md` is the human-facing "how to validate"
  doc referenced by product.md's problem statement.

## Testing strategy
This is a documentation-only change; there is no code path to unit test.
Validate the change by:
1. Re-reading `script/presubmit`, `.github/workflows/CI.yml`, and
   `.agents/skills/test-local-warp/SKILL.md` at implementation time and
   diffing their actual content against every command/behavior claim written
   into `CONTRIBUTING.md`, to catch drift since this spec was written.
2. Running `script/presubmit` locally after making the change, to confirm
   the documentation change itself doesn't break JSON formatting, `cargo
   fmt`, clippy, or tests (it shouldn't touch any of those files, but this is
   a cheap, real check that nothing else was accidentally modified).
3. Manually confirming every file path and command mentioned in the new doc
   exists/is spelled correctly (e.g. `script/presubmit`, `rust-toolchain.toml`,
   `.github/workflows/CI.yml`).
No new automated tests are introduced or required by this change.
