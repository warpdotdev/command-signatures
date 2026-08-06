use super::common;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Parses `tcpdump --list-interfaces` output into interface-name suggestions.
///
/// Each line has the shape `<index>.<name>[ (<description>)][ [<status flags>]]`, and the
/// name is what `-i` takes. The description and the status flags are folded into the
/// suggestion's description so pseudo-devices such as `any` are recognisable and a
/// disconnected interface is visible before it is picked. Lines that do not start with an
/// index — diagnostics from an old tcpdump that writes them to stdout — are skipped.
fn parse_interfaces(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let (index, rest) = line.trim().split_once('.')?;
            index.parse::<u32>().ok()?;
            let rest = rest.trim_start();
            let name = rest.split_whitespace().next()?;
            Some(Suggestion::with_description(
                name,
                interface_description(&rest[name.len()..]),
            ))
        })
        .collect_ordered_results()
}

/// Describes an interface from the part of its `--list-interfaces` line that follows the
/// name, falling back to a generic label when tcpdump reports neither a description nor a
/// meaningful status.
fn interface_description(details: &str) -> String {
    match (parenthesized(details), status_flags(details)) {
        (Some(description), Some(status)) => format!("{description} ({status})"),
        (Some(description), None) => description.to_string(),
        (None, Some(status)) => status.to_string(),
        (None, None) => "Network interface".to_string(),
    }
}

/// Returns the description tcpdump prints in parentheses after the interface name.
///
/// It can itself contain parentheses — `nflog (Linux netfilter log (NFLOG) interface)` — so
/// it runs from the first `(` to the last `)`.
fn parenthesized(details: &str) -> Option<&str> {
    let (_, after_open) = details.split_once('(')?;
    let (description, _) = after_open.rsplit_once(')')?;
    non_empty(description)
}

/// Returns the bracketed status flags that close the line, dropping the `none` tcpdump
/// prints for an interface it knows nothing about.
fn status_flags(details: &str) -> Option<&str> {
    let (_, after_open) = details.rsplit_once('[')?;
    let (status, _) = after_open.split_once(']')?;
    non_empty(status).filter(|status| *status != "none")
}

fn non_empty(text: &str) -> Option<&str> {
    let text = text.trim();
    (!text.is_empty()).then_some(text)
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tcpdump")
        .add_generator(
            "interfaces",
            Generator::script(
                // tcpdump is the authority on what it can capture on: `-D` reports the
                // pseudo-devices (`any`, `nflog`, …) that a kernel interface listing omits.
                // Ignoring stderr keeps a missing binary or an unreadable capture device from
                // turning into suggestions.
                CommandBuilder::single_command_and_ignore_stderr("tcpdump --list-interfaces"),
                parse_interfaces,
            ),
        )
        .add_generator("user_name", common::users_generator())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
