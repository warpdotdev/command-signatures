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

/// `--dump --dump-format json` is the richest source — it carries doc comments and marks private
/// recipes — but it needs just 1.13 or newer, and older releases gate it behind `--unstable`. The
/// `--list` and `--summary` tiers cover those older releases. A tier is only reached when the
/// previous one exits non-zero, which also covers `just` being absent and there being no justfile:
/// every tier fails, stdout stays empty, and the generator yields no suggestions.
const RECIPES_COMMAND: &str = "sh -c \"just --unstable --dump --dump-format json 2>/dev/null || just --list 2>/dev/null || just --summary 2>/dev/null\"";

const VARIABLES_COMMAND: &str = "sh -c \"just --variables 2>/dev/null\"";

/// The subset of `just --dump --dump-format json` output that drives recipe suggestions.
#[derive(Deserialize)]
struct JustfileDump {
    #[serde(default)]
    recipes: BTreeMap<String, DumpRecipe>,
}

#[derive(Deserialize)]
struct DumpRecipe {
    name: String,
    #[serde(default)]
    doc: Option<String>,
    /// True for recipes named with a leading underscore or carrying the `[private]` attribute.
    #[serde(default)]
    private: bool,
}

fn recipe_suggestion(name: impl Into<String>, doc: Option<&str>) -> Suggestion {
    let doc = doc.map(str::trim).filter(|doc| !doc.is_empty());
    Suggestion::with_description(name, doc.unwrap_or(RECIPE_FALLBACK_DESCRIPTION))
        .with_priority(RECIPE_PRIORITY)
}

/// Turns whichever tier of [`RECIPES_COMMAND`] produced output into suggestions. The JSON dump is
/// recognized by its leading brace so that malformed JSON yields nothing rather than being
/// misparsed as a recipe listing.
fn recipes_post_process(output: &str) -> GeneratorResults {
    let output = output.trim();
    if output.is_empty() {
        return GeneratorResults::default();
    }

    if output.starts_with('{') {
        parse_dump_json(output)
    } else {
        parse_recipe_listing(output)
    }
}

fn parse_dump_json(output: &str) -> GeneratorResults {
    let dump: JustfileDump = match serde_json::from_str(output) {
        Ok(dump) => dump,
        Err(e) => {
            log::error!("Couldn't parse the justfile JSON dump with error {e}");
            return GeneratorResults::default();
        }
    };

    dump.recipes
        .into_values()
        .filter(|recipe| !recipe.private)
        .map(|recipe| recipe_suggestion(recipe.name, recipe.doc.as_deref()))
        .collect_unordered_results()
}

/// Dispatches between the two textual tiers. `just --list` prints a heading followed by indented
/// entries, while `just --summary` prints a single space-separated line of recipe names.
fn parse_recipe_listing(output: &str) -> GeneratorResults {
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
fn parse_list(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter(|line| is_indented(line))
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with('[') {
                return None;
            }

            // A doc comment is always separated from the recipe's parameters by whitespace, so
            // splitting on " #" leaves a `#` inside a parameter default with its parameter.
            let (signature, doc) = match line.split_once(" #") {
                Some((signature, doc)) => (signature, Some(doc)),
                None => (line, None),
            };

            Some(recipe_suggestion(signature.split_whitespace().next()?, doc))
        })
        .collect_unordered_results()
}

/// Parses `just --summary` output: recipe names on one space-separated line, with no docs.
fn parse_summary(output: &str) -> GeneratorResults {
    output
        .split_whitespace()
        .map(|name| recipe_suggestion(name, None))
        .collect_unordered_results()
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
            Generator::script(
                CommandBuilder::single_command(RECIPES_COMMAND),
                recipes_post_process,
            ),
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
    use super::{recipes_post_process, variables_post_process};

    /// Real `just --dump --dump-format json` output (just 1.58.0) for a justfile with documented,
    /// undocumented, parameterized, grouped, and both flavors of private recipe.
    const DUMP_JSON: &str = r#"{
      "aliases": { "b": { "attributes": [], "name": "b", "target": "build" } },
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
        "deploy": {
          "attributes": [{ "doc": "Deploy to an environment" }],
          "doc": "Deploy to an environment",
          "name": "deploy",
          "parameters": [{ "default": null, "name": "env" }],
          "private": false
        },
        "lint": {
          "attributes": [{ "group": "ci" }], "doc": null, "name": "lint", "parameters": [], "private": false
        }
      },
      "source": "/tmp/justdemo/justfile"
    }"#;

    /// Real `just --list` output for the same justfile.
    const LIST_OUTPUT: &str = r#"Available recipes:
    build          # Build the project [alias: b]
    deploy env     # Deploy to an environment
    test filter="" # Run the test suite
    undocumented

    [ci]
    lint
"#;

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
                ("deploy", "Deploy to an environment"),
                ("lint", "Just recipe"),
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
    fn test_list_output_lists_recipes_and_skips_group_headings() {
        assert_eq!(
            names_and_descriptions(LIST_OUTPUT),
            expected(&[
                ("build", "Build the project [alias: b]"),
                ("deploy", "Deploy to an environment"),
                ("lint", "Just recipe"),
                ("test", "Run the test suite"),
                ("undocumented", "Just recipe"),
            ])
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

    /// Every tier of the recipes command fails when there is no justfile or `just` is missing, so
    /// the generator has to treat empty and unparseable output as "no suggestions".
    #[test]
    fn test_unusable_output_yields_no_suggestions() {
        for output in ["", "   \n", "{ not json"] {
            assert!(
                names_and_descriptions(output).is_empty(),
                "expected no suggestions for {output:?}"
            );
        }
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

    #[test]
    fn test_variables_are_listed() {
        let names: Vec<_> = variables_post_process("RUST_LOG version\n")
            .suggestions
            .into_iter()
            .map(|suggestion| suggestion.exact_string)
            .collect();
        assert_eq!(names, ["RUST_LOG", "version"]);
    }
}
