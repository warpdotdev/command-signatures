# Product specification: option alias pairing convention

## Summary

Define one contributor convention for representing equivalent option spellings in hand-written command specs. Equivalent short, long, and alternate long spellings belong in one `name` array; genuinely different options remain separate entries. Document the convention in the repository's contributor-facing documentation without requiring a corpus-wide migration.

## Problem

The schema accepts an option `name` as either one string or an array of strings, but the repository does not explain when contributors should use each form. Both patterns are common in `command-signatures/json/`: 14,269 option entries use an array and 38,457 use a string. As a result, contributors and reviewers must infer whether a newly added long form should be grouped with an existing short form.

The same ambiguity affects `exclusiveOn`. Existing specs variously reference one alias or every alias of a conflicting option. Although `exclusiveOn` is not enforced by the runtime today, inconsistent references make specs harder to review and would make future enforcement ambiguous.

## Desired behavior

### Representing option aliases

An option entry represents one semantic option. Its `name` field follows these rules:

1. Use a string when the target command and platform expose only one documented spelling.
2. Use one array for every documented spelling that is fully equivalent: the spellings have the same meaning, argument shape, repeatability, and other option metadata. Put short aliases first, followed by long aliases in the command's documented order.
3. Include multiple long synonyms in that same array when they are equivalent. For example, `["-q", "--quiet", "--silent"]` is one option.
4. Use separate option entries when spellings differ in semantics, accepted arguments, applicability, or metadata. Similar descriptions or names alone do not make options aliases.
5. Do not invent aliases. In particular, a BSD/macOS-oriented spec may legitimately retain a short-only string when the targeted command does not provide a GNU long form. A long form should be paired only after its support and intended platform coverage have been verified.

Equivalent aliases must not be duplicated as separate entries merely to give each spelling its own `name` string.

### Referencing aliases from `exclusiveOn`

`exclusiveOn` contains literal option spellings, not logical option identifiers. Each entry must therefore list every alias a user could type for each conflicting semantic option.

For two mutually exclusive paired options, the convention is:

```json
[
    {
        "name": ["-L", "--files-without-match"],
        "exclusiveOn": ["-l", "--files-with-matches"]
    },
    {
        "name": ["-l", "--files-with-matches"],
        "exclusiveOn": ["-L", "--files-without-match"]
    }
]
```

When adding a verified long form to an existing short-only option:

1. Replace the option's scalar `name` with an array containing the existing short form and the new long form.
2. Search the same command or subcommand for every `exclusiveOn` that references the existing short form and add the new long form beside it.
3. Review the changed option's own `exclusiveOn` list and ensure it includes every alias of each conflicting option.
4. Keep mutual relationships symmetric when the command describes the options as mutually exclusive.

These updates are local to the option and its relationships. Contributors are not required to normalize unrelated options or files.

### Documentation location

The canonical convention will live in `README.md`, under the existing "JSON Command Signatures" section. This is the public, human-facing entry point that already explains where hand-written specs live, whereas `AGENTS.md` is architecture-oriented and `.agents/skills/add-command-spec/SKILL.md` is an execution workflow consumed primarily by agents.

The implementation should add a short pointer from the option-authoring portion of `.agents/skills/add-command-spec/SKILL.md` to the canonical README section. This keeps agents on the same workflow without duplicating the rule. No new contributor file is warranted for one focused convention, and `AGENTS.md` should continue to point agents to the repository skills rather than duplicate authoring details.

## User-visible impact

This is contributor documentation. It makes new and edited specs more predictable to author and review, but it does not change completion rendering, parsing, or runtime option exclusivity.

## Acceptance criteria

- `README.md` states when to use a scalar `name`, when to use one alias array, and when similar spellings require separate entries.
- The documentation covers equivalent short/long aliases and multiple long synonyms, with concrete JSON examples.
- The documentation states that `exclusiveOn` must contain every literal alias of each conflicting option.
- The documentation gives the required update sequence for adding a long form to a short-only option, including affected peer `exclusiveOn` lists.
- The documentation explicitly says not to invent GNU long forms for BSD/macOS-oriented specs or other command variants that do not support them.
- Migration of existing specs is explicitly opportunistic; landing the convention does not require a corpus-wide rewrite.
- `.agents/skills/add-command-spec/SKILL.md` points to the canonical README convention without maintaining a second copy.
- The documentation notes that `exclusiveOn` is currently parsed but not enforced and does not promise runtime behavior.
- The implementation changes contributor documentation only; it does not modify command JSON, Rust source, generators, or generated specs.

## Out of scope

- Rewriting existing specs to conform to the convention.
- Adding missing aliases to any command, including the `du` change discussed in #372.
- Changing the JSON schema or runtime `Opt` representation.
- Implementing, validating, or otherwise changing runtime handling of `exclusiveOn`.
- Adding automated lint or formatting enforcement for the convention.

## Open questions

1. Should the runtime eventually carry `exclusiveOn` into `Opt` and enforce it? Recommendation: track this as a separate behavior change because it needs runtime semantics, compatibility analysis, and tests.
2. Should a future presubmit check enforce alias grouping and complete `exclusiveOn` references? Recommendation: first apply the written convention opportunistically and evaluate false positives across platform-specific and imported specs before adding automation.
