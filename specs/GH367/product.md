# Product Spec: Document How to Validate Signature Changes

## Issue
GitHub issue [#367](https://github.com/warpdotdev/command-signatures/issues/367).

## Scope note (title/body mismatch)
The issue title ("Document how repro fields in triage labels map to signature
test coverage") does not correspond to anything in this repository.
`repro:*` labels are part of the Warp-for-OSS factory's own issue-triage
taxonomy; they are not a feature of `command-signatures`, and there is no
"signature test coverage" concept that maps to them anywhere in this codebase.
Triage concluded the title is a copy-paste artifact and scoped the work from
the issue **body** instead, which asks for documentation of how signature
changes are validated before merge. This spec follows that scoping. A
maintainer should confirm this reading during spec review; do not build
anything that tries to reconcile "repro fields" with this repository.

## Problem
Contributors adding or changing command completion signatures have no single,
discoverable explanation of how a change gets validated before it can be
merged. Today the information that exists is either missing or scattered
across contributor-facing and agent-facing sources that don't overlap
cleanly:

- There is no `CONTRIBUTING.md` in the repository at all.
- `README.md` documents the JSON spec layout and the override system, but
  says nothing about validation, CI, or `script/presubmit`.
- `script/presubmit` (repo root) runs the same four checks CI runs, but is
  never mentioned in `README.md`, so a contributor has no way to discover it
  short of browsing the `script/` directory.
- `.github/workflows/CI.yml` runs four checks on every PR (`npm run
  format:check`, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo
  test`), but this is only visible by reading the workflow YAML or watching a
  PR's checks run.
- Generator-backed (dynamic) completions are not covered by CI or `cargo
  test` at all — verifying one requires manually running a local Warp build
  against a local checkout of this repo and exercising the completion in the
  UI. That procedure is fully documented today only in
  `.agents/skills/test-local-warp/SKILL.md` and (briefly, in its validation
  step) `.agents/skills/add-command-spec/SKILL.md`, both written for Warp
  agents, not human contributors. Neither assumes computer-use tooling is
  unavailable, which a human contributor's terminal session obviously is not
  the same as.
- `AGENTS.md` documents the codebase's architecture and testing invariants,
  but is explicitly agent-oriented (per its own first line: "This file
  provides guidance to agents when working with code in this repository.").

The result: a human contributor can open a PR, get surprised by CI failures
they had no way to anticipate, and has no path at all to verify a generator
they wrote actually produces reasonable completions, since that can only be
checked in a live Warp session.

## Desired behavior
Add contributor-facing documentation that explains, in one place, how a
signature change is expected to be validated before and after opening a PR.
It must cover exactly three things, matching the scope the maintainer gated:

1. Running `script/presubmit` locally before opening a PR, and what it
   checks.
2. What CI checks automatically on every PR (`.github/workflows/CI.yml`).
3. How to manually verify a new or changed generator end-to-end against a
   local Warp build, written for a human contributor (i.e., not assuming
   Warp's internal agent tooling, computer-use automation, or access to
   `.agents/skills/`).

### Where the content goes
Recommendation: create a new **`CONTRIBUTING.md`** at the repository root
(sibling to `README.md`), and add a short pointer to it from `README.md`.

Reasoning: GitHub surfaces a root-level `CONTRIBUTING.md` automatically in
its own UI (a banner when opening a PR/issue, and a "Contributing" link on
the repo sidebar), which maximizes the chance a contributor actually sees it
before submitting a PR. It also keeps `README.md` focused on describing the
spec format/override system rather than growing a large process section.

Alternative considered: add a new `## Contributing` section to `README.md`
instead of a new file. This keeps everything in one file contributors are
already reading, at the cost of making `README.md` longer and less focused,
and forgoing GitHub's automatic `CONTRIBUTING.md` surfacing. Flagged as an
open question below — recommend `CONTRIBUTING.md`, but a maintainer may
prefer the simpler single-file option.

### Required content
See `specs/GH367/tech.md` for the exact file(s), section headings, and the
per-section content requirements. In summary, the new document must, at
minimum:

- Tell a contributor to run `script/presubmit` before opening a PR, and list
  the four checks it runs (JSON format check, `cargo fmt --check`, `cargo
  clippy -D warnings`, `cargo test`) with the literal commands.
- Describe the three CI jobs in `.github/workflows/CI.yml` (`format`, `lint`,
  `test`) and state plainly that they run automatically on every PR and
  mirror `script/presubmit`.
- Explain that generator-backed completions are not covered by CI or `cargo
  test`, and walk through the manual verification procedure: pointing a
  local `warp` checkout's `Cargo.toml` at this repo via a path dependency,
  building and running Warp (`cargo run --features fast_dev`), and exercising
  the new/changed completion with Tab in a running Warp session, then
  reverting the `Cargo.toml` change before committing. This is a rewrite of
  `.agents/skills/test-local-warp/SKILL.md` for a human audience, not a
  verbatim copy — it must not assume agent tooling or reference `.agents/`
  paths as if the reader has access to them.
- Note that generator verification is manual and cannot be automated in CI
  today, so a reviewer may ask for evidence (e.g. a screenshot) that a new
  generator was exercised in a real Warp session.

## Acceptance criteria
- A new file exists containing all three required sections (presubmit, CI,
  generator verification), placed per the file-placement decision above
  (`CONTRIBUTING.md` recommended; a `README.md` section is an acceptable
  substitute if a maintainer redirects it there during spec review).
- Every claim about current tooling is accurate and traceable to a real path:
  `script/presubmit`, `.github/workflows/CI.yml`, `package.json`'s
  `format`/`format:check` scripts, `rust-toolchain.toml`.
- The generator-verification section is written for a human contributor: it
  does not tell the reader to invoke `.agents/skills/*`, computer-use tools,
  or any agent-only mechanism, and it does not assume the reader has
  Warp-internal tooling.
- `README.md` links to the new contributor documentation (if a separate
  `CONTRIBUTING.md` is created).
- No behavioral, CI, or script changes are made — this is a documentation-only
  change.

## Out of scope
- Adding, removing, or modifying any CI check in `.github/workflows/CI.yml`.
- Modifying `script/presubmit` or any other script.
- Automating generator verification in CI (it requires a GUI Warp build and
  is not feasible in a headless CI runner; the doc should say this
  explicitly rather than imply it's a gap to be closed).
- Rewriting or replacing `AGENTS.md`; it stays as agent-oriented guidance.
  The new contributor doc may reference it but should not duplicate its
  architecture content wholesale.
- Any change to the CLA process. `README.md` already states the CLA
  requirement and links to https://cla.warp.dev; the new doc should
  cross-reference this rather than restate the legal text.
- Addressing the issue title's "repro fields in triage labels" framing — see
  the scope note above.

## Open questions for the maintainer
1. **File placement**: `CONTRIBUTING.md` (recommended) vs. a `README.md`
   section. Recommendation stated above; please confirm or redirect.
2. **Depth of the generator-verification section**: should it be a full
   rewrite of `.agents/skills/test-local-warp/SKILL.md` for humans (this
   spec's recommendation), or a short pointer saying "see the local-Warp
   testing procedure used internally" without restating the steps? A full
   rewrite is more useful to contributors who don't have access to
   `.agents/skills/` content, but duplicates content that could drift out of
   sync with the skill file over time.
3. **CLA mention**: should the new document repeat the one-line CLA notice
   from `README.md`, or purely link to the "License" section for it? This
   spec recommends a one-line cross-reference (no duplication) but a
   maintainer may prefer contributors see the CLA requirement without having
   to jump files.
