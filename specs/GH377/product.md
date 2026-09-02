# Product specification: `list` command with graceful empty-input handling

## Background
Issue [#377](https://github.com/warpdotdev/command-signatures/issues/377) reported that `command-signatures list --file /tmp/empty.json` panicked on an empty file. That panic cannot be reproduced on this repository's `main` branch because the described command does not exist: this workspace provides the `warp-command-signatures` and `warp-completion-metadata` libraries, and its only binary is the PowerShell spec generator in `command-signatures/src/bin/autogenerate_powershell.rs`.

The maintainer approved treating the report as an enhancement request. This specification defines the `list` capability the reporter expected, rather than a fix for an existing panic.

## Motivation
Maintainers and tooling need a deterministic way to inspect the signatures available in this repository without writing a Rust consumer. The same capability should support validating and inspecting an external signatures document while treating an empty input as a valid, empty collection. User-controlled input must always return a deliberate result or error, never panic.

## Goals
- Add a stable `command-signatures list` CLI surface.
- Use the repository's embedded completion specs by default.
- Allow `--file <PATH>` to list an external document that uses the existing Fig-compatible command schema.
- Define non-panicking behavior and exit codes for empty, missing, and malformed external input.
- Provide deterministic human-readable and machine-readable output.
- Keep source loading and summary generation reusable from Rust.

## Non-goals
- Changing completion behavior or the existing JSON completion specs.
- Editing, generating, validating, or repairing signature files.
- Recursively listing every nested subcommand as its own row.
- Executing generators or shell commands referenced by a signature.
- Merging an external file with embedded assets; `--file` replaces the default source.
- Supporting YAML, TOML, JavaScript Fig specs, directories, URLs, or standard input.
- Replacing `autogenerate_powershell` or changing the PowerShell generation workflow.
- Publishing a versioned standalone package or defining distribution outside normal Cargo binary builds.

## Proposed design

### CLI and library surface
Add a binary target named `command-signatures` to the `warp-command-signatures` crate. Its first subcommand is:

`command-signatures list [--file <PATH>] [--json]`

The binary is intentionally thin. Source loading, conversion, sorting, and summary construction belong in a public library API so Rust callers and CLI tests exercise the same behavior. A subcommand-oriented CLI is preferred over a one-off `list-signatures` binary because it gives future repository utilities one coherent entry point.

Use `clap` with its derive API. Although this adds a dependency to a workspace that currently has no argument parser, it provides conventional help, stable usage errors, and an extensible subcommand model without maintaining a custom parser. `clap`-reported usage errors exit with code 2.

### Signature sources
Without `--file`, `list` summarizes the signatures returned by the existing embedded-assets path exposed through `commands()` in `command-signatures/src/lib.rs`.

With `--file`, the file replaces the embedded source. The accepted UTF-8 JSON document forms are:
- One non-empty object matching `warp_completion_metadata::fig_types::Command`, the same schema used by each file under `command-signatures/json/`.
- An array of zero or more objects matching that schema, for callers that need one portable collection.
- The exact empty object `{}`, treated as an empty collection for graceful compatibility with common empty JSON documents.
- An empty or whitespace-only file, treated as an empty collection.

Each command object's `name` field may retain its existing one-or-many representation. Conversion therefore may produce more than one listing row from one object. A non-empty object that does not satisfy the existing `Command` schema is malformed; the empty-object exception must not weaken validation of other objects. JSON `null`, scalar JSON values, and arrays containing non-command values are malformed.

### Listing rows
One row represents one top-level `Signature` produced by the existing Fig `Command` conversion. It contains:
- `name`: the command name.
- `description`: the top-level description, or no value when absent.
- `subcommand_count`: the number of immediate subcommands.

Rows are sorted case-insensitively by `name`, with the original name as a tie-breaker, so output is deterministic even though embedded loading is parallel. Nested subcommands are counted but not emitted as separate rows. Descriptions in text output replace tabs, carriage returns, and newlines with spaces so each signature occupies exactly one line.

Default text output starts with the tab-separated header `NAME	SUBCOMMANDS	DESCRIPTION`, followed by one tab-separated row per signature. Missing descriptions render as an empty final field.

`--json` writes a JSON array to standard output. Each element has exactly `name` (string), `description` (string or `null`), and `subcommand_count` (non-negative integer). JSON uses the same ordering as text output. No headings or status prose are mixed into JSON output.

### No-results messaging
An empty result is successful. In text mode, standard output contains exactly `No signatures found.` followed by a newline. In JSON mode, standard output contains exactly `[]` followed by a newline. Standard error is empty in both cases.

### Behavior matrix
| Input | Standard output | Standard error | Exit code |
| --- | --- | --- | --- |
| No `--file`; embedded signatures available | Sorted text table, or JSON array with `--json` | Empty | 0 |
| Empty or whitespace-only file | `No signatures found.` or `[]` | Empty | 0 |
| File containing `[]` | `No signatures found.` or `[]` | Empty | 0 |
| File containing `{}` | `No signatures found.` or `[]` | Empty | 0 |
| Valid command object or command array that converts to zero signatures, such as `{"name":[]}` | `No signatures found.` or `[]` | Empty | 0 |
| Valid command object or non-empty command array | Sorted text table, or JSON array with `--json` | Empty | 0 |
| Malformed JSON or schema-invalid non-empty JSON | Empty | `error: failed to parse signatures file '<PATH>': <REASON>` plus newline | 1 |
| Missing path, directory path, permission failure, or other read error | Empty | `error: failed to read signatures file '<PATH>': <REASON>` plus newline | 1 |
| Invalid CLI arguments or missing option value | Empty apart from any `clap` usage output | `clap` diagnostic and usage | 2 |

`<REASON>` should preserve the useful parser or operating-system explanation without echoing file contents. `<PATH>` is the user-provided display path.

## User-visible acceptance criteria
1. Running `command-signatures list` emits a deterministic list sourced from embedded assets and exits 0.
2. Every text row includes a command name, immediate subcommand count, and description field.
3. `command-signatures list --json` emits only a valid JSON array using the documented fields and ordering.
4. `--file` accepts one existing Fig-compatible command object or an array of such objects and does not merge them with embedded assets.
5. Empty bytes, whitespace-only bytes, `[]`, `{}`, and a valid document that converts to no names all produce the documented no-results output and exit 0.
6. A malformed or schema-invalid file writes a clear parse error to standard error, emits no standard output, and exits 1.
7. A nonexistent or unreadable path writes a clear read error to standard error, emits no standard output, and exits 1.
8. No external-file case panics or prints a Rust panic diagnostic.
9. Listing external data never runs referenced generators or other shell commands.
10. Existing embedded-signature invariants and the PowerShell generator continue to pass unchanged.

## Open questions
- **External collection syntax:** This specification recommends accepting both a single existing `Command` object and an array of `Command` objects. Maintainers may choose to restrict the first release to one object per file, but doing so would make `[]` an error rather than the useful empty collection requested here.
- **Empty object compatibility:** This specification recommends treating only the exact empty object `{}` as no signatures found. Maintainers may instead classify it as schema-invalid for stricter consistency, but that would make two common representations of an empty JSON collection behave differently.
