# Technical specification: option alias pairing convention

## Current implementation

`CommandOption.name` accepts one string or many and deserializes both forms into `Vec<String>` in `completion-metadata/src/fig_types.rs:167`. `CommandOption.exclusive_on` is independently deserialized as `Vec<String>` in `completion-metadata/src/fig_types.rs:190`.

When a `CommandOption` becomes a runtime `Opt`, every value in `name` is copied to `Opt.exact_string`, but `exclusive_on` is not copied (`completion-metadata/src/fig_types.rs:485`). Runtime short- and long-hand collections then classify each value in `Opt.exact_string` with `is_short_hand_flag` and `is_long_hand_flag` (`completion-metadata/src/signature.rs:94` and `completion-metadata/src/signature.rs:101`).

The repository demonstrates both intended alias grouping and the undocumented inconsistency:

- `command-signatures/json/head.json:10` groups equivalent short and long spellings; `command-signatures/json/head.json:30` includes the multi-long `-q`/`--quiet`/`--silent` option.
- `command-signatures/json/grep.json:101` references both aliases of a paired conflicting option in `exclusiveOn`.
- `command-signatures/json/du.json:89` defines `-d` as a short-only option and peer relationships refer only to `-d`.
- `command-signatures/json/df.json:12` combines aliases in one option while its exclusivity relationships use literal individual spellings.
- `README.md`, `AGENTS.md`, and `.agents/skills/add-command-spec/SKILL.md` do not currently define an alias-pairing convention.

No runtime code reads `exclusive_on` after deserialization. The only Rust construction outside the type itself initializes it as empty in `command-signatures/src/powershell_autogenerator/to_fig_types.rs:141`.

## Documentation changes

Implementation is limited to two documentation edits:

1. Add an "Option aliases and exclusivity" subsection to `README.md` beneath "JSON Command Signatures". It will be the canonical source and include:
   - the scalar-versus-array decision rules;
   - short/long and multi-long examples;
   - the criteria for separate entries;
   - the complete-alias rule for `exclusiveOn`;
   - the steps for adding a long form to a short-only option;
   - the platform-support caveat and opportunistic-migration policy;
   - a note that `exclusiveOn` is currently unenforced.
2. Add a concise link in Step 2 of `.agents/skills/add-command-spec/SKILL.md` directing option authors to the README subsection before creating or editing options.

`AGENTS.md` remains unchanged because it already directs agents editing command signatures to repository skills. A new contributor document would add another discovery path and split a convention that fits the README's existing hand-written-spec overview.

## Data and API impact

There are no schema, serialization, API, or runtime data changes. The documented array form is already accepted, and the convention does not alter how `Opt.exact_string` is populated or how flags are classified.

## Edge cases

- **Multiple aliases of one kind:** Multiple short aliases or multiple long aliases can share an array when all option behavior and metadata are identical.
- **Similar but non-equivalent options:** Keep them separate if any argument, semantic, availability, or metadata distinction exists.
- **Platform-specific commands:** Document only aliases verified for the spec's intended command variant. The convention does not make GNU aliases mandatory for BSD/macOS targets.
- **Mixed-platform specs:** If an alias is not valid across the spec's intended coverage, contributors must first make an explicit scope decision rather than infer an alias from another implementation.
- **Existing inconsistency:** Do not touch unrelated entries. Normalize an existing entry only when it is already being changed and the command's behavior is verified.
- **Mutual exclusion:** List all aliases on both sides when exclusivity is bidirectional; do not infer symmetry when the command's behavior is directional.
- **Currently unenforced metadata:** Treat complete `exclusiveOn` lists as authored metadata and future-compatible documentation, not as evidence that Warp currently suppresses or rejects conflicting options.

## Validation strategy

Because implementation is documentation-only:

1. Review every JSON example for valid syntax and consistency with the stated rules.
2. Confirm README anchors and the skill's relative link resolve in the repository.
3. Run `git diff --check` to catch whitespace errors.
4. Confirm the implementation diff contains only `README.md` and `.agents/skills/add-command-spec/SKILL.md`.
5. No Rust build, command-spec formatting, generator verification, or UI test is required unless implementation expands beyond the approved documentation scope.
