//! Focused valid/invalid fixture coverage for the duplicate-short-flag validation rule
//! (https://github.com/warpdotdev/command-signatures/issues/400).
//!
//! Fixtures live under `tests/fixtures/duplicate_short_flags/`.

use warp_completion_metadata::fig_types::Command;
use warp_completion_metadata::validation::{find_short_flag_conflicts, ShortFlagConflict};

fn load(fixture_name: &str) -> Command {
    let path = format!(
        "{}/tests/fixtures/duplicate_short_flags/{fixture_name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let json = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {path}: {err}"));
    serde_json::from_str(&json)
        .unwrap_or_else(|err| panic!("fixture {path} failed to deserialize: {err}"))
}

fn conflicts_for(fixture_name: &str) -> Vec<ShortFlagConflict> {
    find_short_flag_conflicts(&load(fixture_name))
}

#[test]
fn two_distinct_options_conflict() {
    let conflicts = conflicts_for("invalid_two_distinct_options.json");
    assert_eq!(conflicts.len(), 1);
    let conflict = &conflicts[0];
    assert_eq!(conflict.command_path, vec!["flutter", "assemble"]);
    assert_eq!(conflict.flag, "-d");
    assert_eq!(
        conflict
            .claimants
            .iter()
            .map(|c| c.index)
            .collect::<Vec<_>>(),
        vec![0, 2]
    );
    assert_eq!(
        conflict.describe("command-signatures/json/flutter.json"),
        "command-signatures/json/flutter.json: command \"flutter assemble\": duplicate short flag \"-d\" is used by options #1 [\"-d\", \"--device-id\"] and #3 [\"-d\", \"--define\"]"
    );
}

#[test]
fn identical_name_arrays_are_distinguished_by_position() {
    let conflicts = conflicts_for("invalid_identical_name_arrays.json");
    assert_eq!(conflicts.len(), 1);
    assert_eq!(
        conflicts[0]
            .claimants
            .iter()
            .map(|c| c.index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(
        conflicts[0].describe("tool.json"),
        "tool.json: command \"tool\": duplicate short flag \"-v\" is used by options #1 [\"-v\", \"--verbose\"] and #2 [\"-v\", \"--verbose\"]"
    );
}

#[test]
fn three_claimants_produce_one_diagnostic_listing_all_three() {
    let conflicts = conflicts_for("invalid_three_claimants.json");
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].claimants.len(), 3);
    assert_eq!(
        conflicts[0].describe("tool.json"),
        "tool.json: command \"tool\": duplicate short flag \"-t\" is used by options #1 [\"-t\", \"--type-a\"], #2 [\"-t\", \"--type-b\"], and #3 [\"-t\", \"--type-c\"]"
    );
}

#[test]
fn nested_subcommand_conflict_reports_full_root_to_leaf_path() {
    let conflicts = conflicts_for("invalid_nested_subcommand.json");
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].command_path, vec!["tool", "group", "leaf"]);
}

#[test]
fn repeated_internal_spelling_counts_the_option_once() {
    let conflicts = conflicts_for("invalid_repeated_internal_spelling.json");
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].claimants.len(), 2);
    assert_eq!(
        conflicts[0]
            .claimants
            .iter()
            .map(|c| c.index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
}

#[test]
fn html_escaped_spelling_conflicts_with_literal_form() {
    let conflicts = conflicts_for("invalid_html_escaped_vs_literal.json");
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].flag, "-h");
    assert_eq!(
        conflicts[0].describe("tool.json"),
        "tool.json: command \"tool\": duplicate short flag \"-h\" is used by options #1 [\"&#45;h\", \"--help\"] and #2 [\"-h\", \"--history\"]"
    );
}

#[test]
fn command_alias_names_share_one_namespace() {
    let conflicts = conflicts_for("invalid_command_alias_namespace.json");
    assert_eq!(conflicts.len(), 1);
    // The conflict is reported once (not once per alias), and the canonical path uses only the
    // first declared name.
    assert_eq!(conflicts[0].command_path, vec!["foo"]);
}

#[test]
fn distinct_short_flags_do_not_conflict() {
    assert!(conflicts_for("valid_distinct_short_flags.json").is_empty());
}

#[test]
fn parent_and_child_may_reuse_a_short_flag() {
    assert!(conflicts_for("valid_root_and_child_reuse.json").is_empty());
}

#[test]
fn sibling_subcommands_may_reuse_a_short_flag() {
    assert!(conflicts_for("valid_sibling_subcommands_reuse.json").is_empty());
}

#[test]
fn unrelated_top_level_fixtures_may_reuse_a_short_flag() {
    assert!(conflicts_for("valid_unrelated_top_level_a.json").is_empty());
    assert!(conflicts_for("valid_unrelated_top_level_b.json").is_empty());
}

#[test]
fn short_and_long_alias_on_one_option_does_not_conflict() {
    assert!(conflicts_for("valid_short_and_long_alias.json").is_empty());
}

#[test]
fn single_dash_long_names_are_not_short_flags() {
    assert!(conflicts_for("valid_long_flags_not_short.json").is_empty());
}
