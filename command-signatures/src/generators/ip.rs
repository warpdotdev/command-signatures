use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Parses `ip -o link show` output into interface-name suggestions.
///
/// Each line has the shape `<index>: <name>[@<parent>]: <FLAGS> mtu ... state <STATE> ...`.
/// The name is what `dev`/`iif`/`oif`-style arguments take; a `@parent` suffix (used by
/// tunnels and VLANs to show their underlying device) is stripped since it isn't part of
/// the interface name itself. The device's operational state is folded into the
/// suggestion's description so a disconnected or down interface is still recognisable.
fn parse_interfaces(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once(": ")?;
            let (name, details) = rest.split_once(": ")?;
            let name = name.split('@').next()?.trim();
            if name.is_empty() {
                return None;
            }
            Some(Suggestion::with_description(
                name,
                interface_state(details).unwrap_or_else(|| "Network interface".to_string()),
            ))
        })
        .collect_ordered_results()
}

/// Describes an interface from its operational `state` field, e.g. `UP` or `DOWN`.
fn interface_state(details: &str) -> Option<String> {
    let (_, after_state) = details.split_once("state ")?;
    let state = after_state.split_whitespace().next()?;
    Some(format!("Interface ({state})"))
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ip")
        .add_generator(
            "netns",
            Generator::script(
                CommandBuilder::single_command("ip netns list 2>/dev/null"),
                |output| {
                    output
                        .lines()
                        .filter_map(|line| {
                            // Output format: "name" or "name (id: N)"
                            let name = line.split_whitespace().next()?;
                            if name.is_empty() {
                                return None;
                            }
                            Some(Suggestion::with_description(name, "Network namespace"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "network_interfaces",
            Generator::script(
                // `-o` keeps each interface on its own line so it can be parsed reliably.
                CommandBuilder::single_command_and_ignore_stderr("ip -o link show"),
                parse_interfaces,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real `ip -o link show` output on Linux (backslash-continuation lines trimmed).
    const LINUX_OUTPUT: &str = "1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000\\    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00\n2: dummy0: <BROADCAST,NOARP> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000\\    link/ether 66:ba:47:5b:05:c9 brd ff:ff:ff:ff:ff:ff\n3: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP mode DEFAULT group default qlen 1000\\    link/ether aa:fc:00:00:00:01 brd ff:ff:ff:ff:ff:ff\n5: tunl0@NONE: <NOARP> mtu 1480 qdisc noop state DOWN mode DEFAULT group default qlen 1000\\    link/ipip 0.0.0.0 brd 0.0.0.0\n";

    #[test]
    fn test_parse_interfaces_keeps_the_order_ip_reports() {
        let results = parse_interfaces(LINUX_OUTPUT);
        let names: Vec<&str> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.exact_string.as_str())
            .collect();
        assert_eq!(names, vec!["lo", "dummy0", "eth0", "tunl0"]);
        assert!(results.is_ordered);
    }

    #[test]
    fn test_parse_interfaces_describes_by_state() {
        let results = parse_interfaces(LINUX_OUTPUT);
        let descriptions: Vec<Option<&str>> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.description.as_deref())
            .collect();
        assert_eq!(
            descriptions,
            vec![
                Some("Interface (UNKNOWN)"),
                Some("Interface (DOWN)"),
                Some("Interface (UP)"),
                Some("Interface (DOWN)"),
            ]
        );
    }

    #[test]
    fn test_parse_interfaces_strips_the_at_parent_suffix() {
        let results = parse_interfaces(LINUX_OUTPUT);
        assert_eq!(results.suggestions[3].exact_string, "tunl0");
    }

    #[test]
    fn test_parse_interfaces_empty_output() {
        assert!(parse_interfaces("").suggestions.is_empty());
    }
}
