use super::tcpdump::parse_interfaces;

/// Real `tcpdump 4.99.4 --list-interfaces` output on Linux.
const LINUX_OUTPUT: &str = "1.eth0 [Up, Running, Connected]\n2.any (Pseudo-device that captures on all interfaces) [Up, Running]\n3.lo [Up, Running, Loopback]\n4.docker0 [Up, Disconnected]\n5.dummy0 [none]\n";

#[test]
fn test_parse_interfaces_keeps_the_order_tcpdump_reports() {
    let results = parse_interfaces(LINUX_OUTPUT);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();
    assert_eq!(names, vec!["eth0", "any", "lo", "docker0", "dummy0"]);
    assert!(results.is_ordered);
}

#[test]
fn test_parse_interfaces_describes_by_description_and_status() {
    let results = parse_interfaces(LINUX_OUTPUT);
    let descriptions: Vec<Option<&str>> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.description.as_deref())
        .collect();
    assert_eq!(
        descriptions,
        vec![
            Some("Up, Running, Connected"),
            Some("Pseudo-device that captures on all interfaces (Up, Running)"),
            Some("Up, Running, Loopback"),
            Some("Up, Disconnected"),
            // A "[none]" status carries no information, so the generic label is used.
            Some("Network interface"),
        ]
    );
}

#[test]
fn test_parse_interfaces_keeps_nested_parentheses_in_the_description() {
    let results = parse_interfaces("10.nflog (Linux netfilter log (NFLOG) interface) [none]\n");
    assert_eq!(results.suggestions[0].exact_string, "nflog");
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("Linux netfilter log (NFLOG) interface")
    );
}

#[test]
fn test_parse_interfaces_without_status_flags() {
    // Older tcpdump releases print the name alone.
    let results = parse_interfaces("1.en0\n2.awdl0\n");
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();
    assert_eq!(names, vec!["en0", "awdl0"]);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("Network interface")
    );
}

#[test]
fn test_parse_interfaces_skips_lines_without_an_index() {
    let output =
        "tcpdump: You don't have permission to perform this capture\n\n1.eth0 [Up, Running]\n";
    let results = parse_interfaces(output);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "eth0");
}

#[test]
fn test_parse_interfaces_empty_output() {
    assert!(parse_interfaces("").suggestions.is_empty());
}
