use serde::Deserialize;
use std::collections::BTreeMap;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Importance, Order, Priority, Suggestion,
};

/// Description shown for a recipe that has no doc comment.
const RECIPE_FALLBACK_DESCRIPTION: &str = "Just recipe";
const VARIABLE_DESCRIPTION: &str = "Just variable";

/// Recipes outrank the file and folder suggestions offered for the same position, since a bare
/// `just <TAB>` is asking for a recipe rather than a path.
const RECIPE_PRIORITY: Priority = Priority::Global(Importance::More(Order(80)));

/// `--dump --dump-format json` is the richest source — it carries doc comments, marks private
/// recipes, and declares each recipe's parameters — but it needs just 1.13 or newer, and older
/// releases gate it behind `--unstable`. The `--list` and `--summary` tiers cover those older
/// releases. A tier is only reached when the previous one exits non-zero, which also covers `just`
/// being absent and there being no justfile: every tier fails, no recipe data follows the context
/// line, and the generator yields no suggestions.
const RECIPE_SOURCES: &str = "just --unstable --dump --dump-format json 2>/dev/null || just --list 2>/dev/null || just --summary 2>/dev/null";

const VARIABLES_COMMAND: &str = "sh -c \"just --variables 2>/dev/null\"";

/// Marks the line the recipes command prints ahead of the recipe data, carrying the arguments
/// already on the command line. A generator callback receives only its command's output, so the
/// command has to relay the surrounding tokens for the callback to tell a recipe position from a
/// parameter position.
const CONTEXT_PREFIX: &str = "warp-just-context:";

/// Stands in for a command-line token that cannot name a recipe. Emitting only recipe-name
/// characters keeps arbitrary command-line text out of the generated shell command, and `-` is not
/// a legal recipe name, so it never matches one.
const OPAQUE_TOKEN: &str = "-";

/// Caps how much of the command line the context carries. A line longer than this is replayed from
/// its middle in [`is_recipe_position`], which can misread which recipe owns the cursor — a length
/// no realistic `just` invocation reaches.
const MAX_CONTEXT_TOKENS: usize = 32;

/// How many arguments a recipe binds. `just` binds greedily, taking up to the recipe's declared
/// parameter count before treating the next token as another recipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arity {
    /// The recipe declares this many parameters. Parameters carrying a default are included, since
    /// `just` still binds an argument to them when one is supplied.
    Bounded(usize),
    /// A `+`/`*` parameter absorbs every remaining argument, so no later token starts a recipe.
    Unbounded,
}

/// The recipes a justfile declares: the suggestions offered at a recipe position, plus the arity of
/// every name that can appear on the command line — including private recipes and aliases, which
/// are never suggested but are still invocable.
#[derive(Default)]
struct Recipes {
    suggestions: Vec<Suggestion>,
    arities: BTreeMap<String, Arity>,
}

impl Recipes {
    fn add(&mut self, name: String, arity: Arity, suggestion: Option<Suggestion>) {
        self.suggestions.extend(suggestion);
        self.arities.insert(name, arity);
    }
}

/// The subset of `just --dump --dump-format json` output that drives recipe suggestions.
#[derive(Deserialize)]
struct JustfileDump {
    #[serde(default)]
    recipes: BTreeMap<String, DumpRecipe>,
    /// An alias invokes its target recipe, so it binds the target's arguments.
    #[serde(default)]
    aliases: BTreeMap<String, DumpAlias>,
}

#[derive(Deserialize)]
struct DumpRecipe {
    name: String,
    #[serde(default)]
    doc: Option<String>,
    /// True for recipes named with a leading underscore or carrying the `[private]` attribute.
    #[serde(default)]
    private: bool,
    #[serde(default)]
    parameters: Vec<DumpParameter>,
}

#[derive(Deserialize)]
struct DumpParameter {
    /// `plus` for `+name` and `star` for `*name`. The singular parameters that make up the common
    /// case, and releases predating the field, leave it `"singular"` or absent.
    #[serde(default)]
    kind: Option<String>,
}

#[derive(Deserialize)]
struct DumpAlias {
    name: String,
    target: String,
}

impl DumpParameter {
    fn is_variadic(&self) -> bool {
        matches!(self.kind.as_deref(), Some("plus" | "star"))
    }
}

fn arity_of(parameters: &[DumpParameter]) -> Arity {
    if parameters.iter().any(DumpParameter::is_variadic) {
        Arity::Unbounded
    } else {
        Arity::Bounded(parameters.len())
    }
}

fn recipe_suggestion(name: impl Into<String>, doc: Option<&str>) -> Suggestion {
    let doc = doc.map(str::trim).filter(|doc| !doc.is_empty());
    Suggestion::with_description(name, doc.unwrap_or(RECIPE_FALLBACK_DESCRIPTION))
        .with_priority(RECIPE_PRIORITY)
}

/// Recipe names are `just` identifiers: an ASCII letter or underscore followed by letters, digits,
/// underscores, or dashes.
fn is_recipe_name(token: &str) -> bool {
    let mut characters = token.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// The arguments already committed before the cursor, with the `just` invocation itself dropped.
/// Without trailing whitespace the final token is the partial word being completed, which is not
/// yet an argument.
fn preceding_arguments<'a>(tokens: &'a [&'a str], has_trailing_whitespace: bool) -> &'a [&'a str] {
    let committed = if has_trailing_whitespace {
        tokens
    } else {
        &tokens[..tokens.len().saturating_sub(1)]
    };
    committed.get(1..).unwrap_or_default()
}

fn context_line(preceding: &[&str]) -> String {
    let start = preceding.len().saturating_sub(MAX_CONTEXT_TOKENS);
    let tokens: Vec<&str> = preceding[start..]
        .iter()
        .map(|token| {
            if is_recipe_name(token) {
                *token
            } else {
                OPAQUE_TOKEN
            }
        })
        .collect();
    format!("{CONTEXT_PREFIX} {}", tokens.join(" "))
}

fn recipes_command(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    _environment: &[String],
) -> CommandBuilder {
    let context = context_line(preceding_arguments(tokens, has_trailing_whitespace));
    CommandBuilder::single_command(format!("sh -c \"echo '{context}'; {RECIPE_SOURCES}\""))
}

/// Splits the context line the command prepends from the recipe data that follows it. Output with
/// no context line is all recipe data, which places the cursor at the first argument.
fn split_context(output: &str) -> (Vec<&str>, &str) {
    match output.trim_start().strip_prefix(CONTEXT_PREFIX) {
        Some(rest) => {
            let (context, recipe_data) = rest.split_once('\n').unwrap_or((rest, ""));
            (context.split_whitespace().collect(), recipe_data)
        }
        None => (Vec::new(), output),
    }
}

/// Decides whether the cursor sits where a recipe name belongs, by replaying how `just` binds the
/// arguments already on the command line: each recipe greedily takes up to its declared parameter
/// count, and whatever follows starts the next recipe. Tokens no recipe claims are `just`'s own
/// options and their values, which are stepped over. The cursor belongs to a recipe rather than to
/// a new one when the last recipe still has parameters left to fill, or when a variadic parameter
/// has claimed the rest of the line.
fn is_recipe_position(preceding: &[&str], arities: &BTreeMap<String, Arity>) -> bool {
    let mut position = 0;
    while position < preceding.len() {
        match arities.get(preceding[position]) {
            Some(Arity::Unbounded) => return false,
            Some(Arity::Bounded(parameters)) => {
                let arguments_available = preceding.len() - position - 1;
                if arguments_available < *parameters {
                    return false;
                }
                position += 1 + parameters;
            }
            None => position += 1,
        }
    }
    true
}

/// Turns whichever tier of [`RECIPE_SOURCES`] produced output into suggestions, offering them only
/// at a recipe position. The JSON dump is recognized by its leading brace so that malformed JSON
/// yields nothing rather than being misparsed as a recipe listing.
fn recipes_post_process(output: &str) -> GeneratorResults {
    let (preceding, recipe_data) = split_context(output);
    let recipe_data = recipe_data.trim();
    if recipe_data.is_empty() {
        return GeneratorResults::default();
    }

    let recipes = if recipe_data.starts_with('{') {
        parse_dump_json(recipe_data)
    } else {
        parse_recipe_listing(recipe_data)
    };

    if !is_recipe_position(&preceding, &recipes.arities) {
        return GeneratorResults::default();
    }

    recipes.suggestions.into_iter().collect_unordered_results()
}

fn parse_dump_json(output: &str) -> Recipes {
    let dump: JustfileDump = match serde_json::from_str(output) {
        Ok(dump) => dump,
        Err(e) => {
            log::error!("Couldn't parse the justfile JSON dump with error {e}");
            return Recipes::default();
        }
    };

    let mut recipes = Recipes::default();
    for recipe in dump.recipes.into_values() {
        let suggestion = (!recipe.private)
            .then(|| recipe_suggestion(recipe.name.as_str(), recipe.doc.as_deref()));
        recipes.add(recipe.name, arity_of(&recipe.parameters), suggestion);
    }

    for alias in dump.aliases.into_values() {
        if let Some(arity) = recipes.arities.get(&alias.target).copied() {
            recipes.arities.insert(alias.name, arity);
        }
    }

    recipes
}

/// Dispatches between the two textual tiers. `just --list` prints a heading followed by indented
/// entries, while `just --summary` prints a single space-separated line of recipe names.
fn parse_recipe_listing(output: &str) -> Recipes {
    let is_list = output
        .lines()
        .next()
        .is_some_and(|heading| heading.trim_end().ends_with(':'))
        || output.lines().any(is_indented);

    if is_list {
        parse_list(output)
    } else {
        parse_summary(output)
    }
}

fn is_indented(line: &str) -> bool {
    line.starts_with([' ', '\t'])
}

/// Parses `just --list` output, whose entries look like `    name PARAM="v" # doc` and are grouped
/// under an unindented heading plus optional `[group]` sub-headings.
fn parse_list(output: &str) -> Recipes {
    let mut recipes = Recipes::default();

    for line in output.lines().filter(|line| is_indented(line)) {
        let line = line.trim();
        if line.starts_with('[') {
            continue;
        }

        // A doc comment is always separated from the recipe's parameters by whitespace, so
        // splitting on " #" leaves a `#` inside a parameter default with its parameter.
        let (signature, doc) = match line.split_once(" #") {
            Some((signature, doc)) => (signature, Some(doc)),
            None => (line, None),
        };

        let mut words = signature.split_whitespace();
        let Some(name) = words.next() else {
            continue;
        };

        // A quoted default containing whitespace reads as several parameters, which only widens the
        // span the recipe claims by a position or two.
        let parameters: Vec<&str> = words.collect();
        let arity = if parameters
            .iter()
            .any(|parameter| parameter.starts_with(['+', '*']))
        {
            Arity::Unbounded
        } else {
            Arity::Bounded(parameters.len())
        };

        recipes.add(name.to_string(), arity, Some(recipe_suggestion(name, doc)));
    }

    recipes
}

/// Parses `just --summary` output: recipe names on one space-separated line, with no docs. The tier
/// reveals no parameters, so every recipe is treated as taking none and recipe names keep being
/// offered where the JSON dump would suppress them.
fn parse_summary(output: &str) -> Recipes {
    let mut recipes = Recipes::default();
    for name in output.split_whitespace() {
        recipes.add(
            name.to_string(),
            Arity::Bounded(0),
            Some(recipe_suggestion(name, None)),
        );
    }
    recipes
}

/// Parses `just --variables` output: variable names on one space-separated line.
fn variables_post_process(output: &str) -> GeneratorResults {
    output
        .split_whitespace()
        .map(|name| Suggestion::with_description(name, VARIABLE_DESCRIPTION))
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("just")
        .add_generator(
            "recipes",
            Generator::command_from_tokens(recipes_command, recipes_post_process),
        )
        .add_generator(
            "variables",
            Generator::script(
                CommandBuilder::single_command(VARIABLES_COMMAND),
                variables_post_process,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::{
        context_line, preceding_arguments, recipes_command, recipes_post_process,
        variables_post_process, MAX_CONTEXT_TOKENS,
    };
    use warp_completion_metadata::Shell;

    /// Real `just --dump --dump-format json` output (just 1.58.0), trimmed to the fields the
    /// generator reads, for a justfile with documented, undocumented, parameterized, defaulted,
    /// variadic, aliased, grouped, and both flavors of private recipe.
    const DUMP_JSON: &str = r#"{
      "aliases": {
        "b": { "attributes": [], "name": "b", "target": "build" },
        "d": { "attributes": [], "name": "d", "target": "deploy" }
      },
      "assignments": {
        "version": { "eager": false, "export": false, "name": "version", "private": false, "value": "1.0.0" }
      },
      "first": "build",
      "recipes": {
        "_private-helper": {
          "attributes": [], "doc": null, "name": "_private-helper", "parameters": [], "private": true
        },
        "attr-private": {
          "attributes": ["private"], "doc": null, "name": "attr-private", "parameters": [], "private": true
        },
        "build": {
          "attributes": [], "doc": "Build the project", "name": "build", "parameters": [], "private": false
        },
        "clean": {
          "attributes": [],
          "doc": "Clean paths",
          "name": "clean",
          "parameters": [{ "default": null, "kind": "star", "name": "paths" }],
          "private": false
        },
        "deploy": {
          "attributes": [{ "doc": "Deploy to an environment" }],
          "doc": "Deploy to an environment",
          "name": "deploy",
          "parameters": [{ "default": null, "kind": "singular", "name": "env" }],
          "private": false
        },
        "lint": {
          "attributes": [{ "group": "ci" }], "doc": null, "name": "lint", "parameters": [], "private": false
        },
        "package": {
          "attributes": [],
          "doc": "Package everything",
          "name": "package",
          "parameters": [
            { "default": null, "kind": "singular", "name": "target" },
            { "default": null, "kind": "plus", "name": "files" }
          ],
          "private": false
        },
        "release": {
          "attributes": [],
          "doc": "Release",
          "name": "release",
          "parameters": [
            { "default": null, "kind": "singular", "name": "version" },
            { "default": "patch", "kind": "singular", "name": "bump" }
          ],
          "private": false
        }
      },
      "source": "/tmp/justdemo/justfile"
    }"#;

    /// Real `just --list` output for the same justfile.
    const LIST_OUTPUT: &str = r#"Available recipes:
    build                        # Build the project [alias: b]
    clean *paths                 # Clean paths
    deploy env                   # Deploy to an environment
    package target +files        # Package everything
    release version bump="patch" # Release
    test filter=""               # Run the test suite
    undocumented

    [ci]
    lint
"#;

    const DUMP_RECIPE_NAMES: [&str; 6] = ["build", "clean", "deploy", "lint", "package", "release"];

    /// Mimics what the generator receives: the context line [`recipes_command`] prints for the
    /// tokens on the command line, followed by the recipe data one of its tiers produced.
    fn output_for(tokens: &[&str], has_trailing_whitespace: bool, recipe_data: &str) -> String {
        let context = context_line(preceding_arguments(tokens, has_trailing_whitespace));
        format!("{context}\n{recipe_data}")
    }

    fn recipe_names(output: &str) -> Vec<String> {
        let mut names: Vec<String> = recipes_post_process(output)
            .suggestions
            .into_iter()
            .map(|suggestion| suggestion.exact_string)
            .collect();
        names.sort();
        names
    }

    /// The recipe names offered at the fresh word that follows `tokens`.
    fn suggestions_after(tokens: &[&str], recipe_data: &str) -> Vec<String> {
        recipe_names(&output_for(tokens, true, recipe_data))
    }

    fn names_and_descriptions(output: &str) -> Vec<(String, Option<String>)> {
        let mut results: Vec<_> = recipes_post_process(output)
            .suggestions
            .into_iter()
            .map(|suggestion| (suggestion.exact_string, suggestion.description))
            .collect();
        results.sort();
        results
    }

    fn expected(pairs: &[(&str, &str)]) -> Vec<(String, Option<String>)> {
        pairs
            .iter()
            .map(|(name, description)| (name.to_string(), Some(description.to_string())))
            .collect()
    }

    #[test]
    fn test_dump_json_lists_public_recipes_with_docs() {
        assert_eq!(
            names_and_descriptions(DUMP_JSON),
            expected(&[
                ("build", "Build the project"),
                ("clean", "Clean paths"),
                ("deploy", "Deploy to an environment"),
                ("lint", "Just recipe"),
                ("package", "Package everything"),
                ("release", "Release"),
            ])
        );
    }

    #[test]
    fn test_dump_json_recipes_outrank_path_suggestions() {
        let suggestions = recipes_post_process(DUMP_JSON).suggestions;
        assert!(suggestions
            .iter()
            .all(|suggestion| suggestion.priority.is_global()));
    }

    #[test]
    fn test_recipes_are_offered_at_the_first_argument_position() {
        assert_eq!(suggestions_after(&["just"], DUMP_JSON), DUMP_RECIPE_NAMES);
    }

    /// The reported bug: `deploy env:` takes a parameter, so `just deploy <TAB>` is completing that
    /// parameter and another recipe name is never a valid answer there.
    #[test]
    fn test_recipes_are_not_offered_at_a_parameter_position() {
        assert!(suggestions_after(&["just", "deploy"], DUMP_JSON).is_empty());
    }

    /// Recipes can be chained, so once a recipe's parameters are filled the next token starts a new
    /// recipe.
    #[test]
    fn test_recipes_are_offered_again_once_the_parameters_are_filled() {
        assert_eq!(
            suggestions_after(&["just", "deploy", "prod"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    #[test]
    fn test_a_recipe_without_parameters_is_immediately_followed_by_another_recipe() {
        assert_eq!(
            suggestions_after(&["just", "build"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    /// A defaulted parameter still binds an argument when one is supplied, so it widens the span the
    /// recipe claims.
    #[test]
    fn test_a_defaulted_parameter_counts_toward_the_arity() {
        assert!(suggestions_after(&["just", "release", "1.0.0"], DUMP_JSON).is_empty());
        assert_eq!(
            suggestions_after(&["just", "release", "1.0.0", "minor"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    /// `*paths` and `+files` absorb every remaining argument, so no later token starts a recipe.
    #[test]
    fn test_a_variadic_parameter_claims_every_remaining_position() {
        for tokens in [
            vec!["just", "clean"],
            vec!["just", "clean", "dist", "target", "build"],
            vec!["just", "package", "linux", "a", "b"],
        ] {
            assert!(
                suggestions_after(&tokens, DUMP_JSON).is_empty(),
                "expected no suggestions after {tokens:?}"
            );
        }
    }

    /// An argument value can coincidentally spell a recipe name, so what a token means depends on
    /// the recipe that claimed it rather than on the name alone.
    #[test]
    fn test_an_argument_that_spells_a_recipe_name_is_still_an_argument() {
        assert!(suggestions_after(&["just", "clean", "dist", "build"], DUMP_JSON).is_empty());
        assert_eq!(
            suggestions_after(&["just", "deploy", "build"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    #[test]
    fn test_an_alias_binds_the_arguments_of_its_target() {
        assert!(suggestions_after(&["just", "d"], DUMP_JSON).is_empty());
        assert_eq!(
            suggestions_after(&["just", "b"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    /// A private recipe is hidden from the menu but still invocable, so it has to claim its own
    /// parameter positions.
    #[test]
    fn test_a_private_recipe_is_hidden_but_still_governs_what_follows_it() {
        assert_eq!(
            suggestions_after(&["just", "_private-helper"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    /// `--show` takes a recipe name, and a flag is never a recipe.
    #[test]
    fn test_show_still_completes_recipe_names() {
        assert_eq!(
            suggestions_after(&["just", "--show"], DUMP_JSON),
            DUMP_RECIPE_NAMES
        );
    }

    /// The partial word under the cursor has not been bound to the recipe yet, so it only counts as
    /// an argument once it is followed by whitespace.
    #[test]
    fn test_a_partial_word_is_bound_to_the_recipe_only_once_it_is_committed() {
        assert!(recipe_names(&output_for(&["just", "deploy", "pr"], false, DUMP_JSON)).is_empty());
        assert_eq!(
            recipe_names(&output_for(&["just", "deploy", "pr"], true, DUMP_JSON)),
            DUMP_RECIPE_NAMES
        );
    }

    #[test]
    fn test_list_output_lists_recipes_and_skips_group_headings() {
        assert_eq!(
            names_and_descriptions(LIST_OUTPUT),
            expected(&[
                ("build", "Build the project [alias: b]"),
                ("clean", "Clean paths"),
                ("deploy", "Deploy to an environment"),
                ("lint", "Just recipe"),
                ("package", "Package everything"),
                ("release", "Release"),
                ("test", "Run the test suite"),
                ("undocumented", "Just recipe"),
            ])
        );
    }

    /// `just --list` spells out each recipe's parameters, so the older tier is arity-aware too.
    #[test]
    fn test_list_output_is_arity_aware() {
        assert!(suggestions_after(&["just", "deploy"], LIST_OUTPUT).is_empty());
        assert!(suggestions_after(&["just", "clean", "dist", "target"], LIST_OUTPUT).is_empty());
        assert_eq!(
            suggestions_after(&["just", "deploy", "prod"], LIST_OUTPUT),
            [
                "build",
                "clean",
                "deploy",
                "lint",
                "package",
                "release",
                "test",
                "undocumented"
            ]
        );
    }

    #[test]
    fn test_list_output_with_only_private_recipes_is_empty() {
        assert!(names_and_descriptions("Available recipes:\n").is_empty());
    }

    #[test]
    fn test_list_output_keeps_a_hash_inside_a_parameter_default() {
        assert_eq!(
            names_and_descriptions("Available recipes:\n    tag prefix=\"#\" # Tag a release\n"),
            expected(&[("tag", "Tag a release")])
        );
    }

    #[test]
    fn test_summary_output_lists_recipes() {
        assert_eq!(
            names_and_descriptions("build deploy lint\n"),
            expected(&[
                ("build", "Just recipe"),
                ("deploy", "Just recipe"),
                ("lint", "Just recipe"),
            ])
        );
    }

    /// `just --summary` carries no parameters at all, so it keeps completing recipe names rather
    /// than suppressing positions it cannot reason about.
    #[test]
    fn test_summary_output_keeps_completing_recipe_names_without_parameter_data() {
        assert_eq!(
            suggestions_after(&["just", "deploy"], "build deploy lint\n"),
            ["build", "deploy", "lint"]
        );
    }

    /// Every tier of the recipes command fails when there is no justfile or `just` is missing, so
    /// the generator has to treat empty and unparseable output as "no suggestions".
    #[test]
    fn test_unusable_output_yields_no_suggestions() {
        for recipe_data in ["", "   \n", "{ not json"] {
            assert!(
                suggestions_after(&["just"], recipe_data).is_empty(),
                "expected no suggestions for {recipe_data:?}"
            );
            assert!(
                names_and_descriptions(recipe_data).is_empty(),
                "expected no suggestions for {recipe_data:?}"
            );
        }
    }

    /// Command-line text reaches the generated shell command, so only tokens that could name a
    /// recipe are relayed and everything else collapses to a placeholder that matches no recipe.
    #[test]
    fn test_context_relays_only_recipe_name_tokens() {
        assert_eq!(
            context_line(&["deploy", "--set", "a b", "$(rm -rf /)", "'", "_x-1"]),
            "warp-just-context: deploy - - - - _x-1"
        );
    }

    #[test]
    fn test_context_is_capped_at_the_most_recent_tokens() {
        let tokens: Vec<&str> = std::iter::once("just")
            .chain(std::iter::repeat_n("filler", MAX_CONTEXT_TOKENS + 5))
            .collect();
        let context = context_line(preceding_arguments(&tokens, true));
        assert_eq!(
            context.split_whitespace().count() - 1,
            MAX_CONTEXT_TOKENS,
            "unexpected context: {context}"
        );
    }

    #[test]
    fn test_recipes_command_carries_the_context_ahead_of_the_recipe_sources() {
        let builder = recipes_command(&["just", "deploy"], true, &[]);
        let command = builder.build(Shell::Posix);
        assert!(
            command.starts_with("sh -c \"echo 'warp-just-context: deploy'; just --unstable"),
            "unexpected command: {command}"
        );
    }

    #[test]
    fn test_variables_are_listed() {
        let names: Vec<_> = variables_post_process("RUST_LOG version\n")
            .suggestions
            .into_iter()
            .map(|suggestion| suggestion.exact_string)
            .collect();
        assert_eq!(names, ["RUST_LOG", "version"]);
    }

    /// The recipe generator only reaches the completion menu if `just.json` points at it; before it
    /// existed the spec carried Fig's JavaScript generators, which Warp ignores, so `just <TAB>`
    /// fell back to filesystem paths.
    #[cfg(feature = "embed-signatures")]
    #[test]
    fn test_just_spec_wires_recipe_arguments_to_the_recipes_generator() {
        use warp_completion_metadata::{ArgumentType, GeneratorName};

        let signature = crate::signature_by_name("just").expect("the just spec should exist");
        let recipes = ArgumentType::Generator(GeneratorName::new("recipes"));

        let recipe_argument = signature
            .arguments()
            .first()
            .expect("just should take recipe arguments");
        assert!(recipe_argument.argument_types.contains(&recipes));

        let show = signature
            .options()
            .iter()
            .find(|option| option.has_name("--show"))
            .expect("just should have a --show option");
        assert!(show
            .arguments()
            .iter()
            .any(|argument| argument.argument_types.contains(&recipes)));
    }
}
