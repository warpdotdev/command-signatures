use super::parse_package_list;

#[test]
fn test_parses_aur_and_repo_packages() {
    let output = "btrfs-progs\tcore\nyay-bin\tAUR\n";
    let results = parse_package_list(output);

    let suggestions: Vec<(&str, Option<&str>)> = results
        .suggestions
        .iter()
        .map(|suggestion| {
            (
                suggestion.exact_string.as_str(),
                suggestion.description.as_deref(),
            )
        })
        .collect();

    assert_eq!(
        suggestions,
        vec![("btrfs-progs", Some("core")), ("yay-bin", Some("AUR"))]
    );
}

#[test]
fn test_handles_single_field_line_without_panicking() {
    // A line with no tab-separated source shouldn't panic, and should still surface the
    // package name (just without a description).
    let output = "btrfs-progs\n";
    let results = parse_package_list(output);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "btrfs-progs");
    assert_eq!(results.suggestions[0].description, None);
}

#[test]
fn test_skips_blank_lines_and_lines_with_no_name() {
    let output = "\n\t\nbtrfs-progs\tcore\n";
    let results = parse_package_list(output);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "btrfs-progs");
}

#[test]
fn test_empty_output() {
    assert!(parse_package_list("").suggestions.is_empty());
}
