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

/// Parses `ip tunnel show` output into tunnel-name suggestions.
///
/// Each configured tunnel gets its own line, shaped `<name>: <mode>/ip remote <addr> local
/// <addr> ttl <ttl> [nopmtudisc] ...` (e.g. `gre1: gre/ip remote 10.0.0.2 local 10.0.0.1 ttl
/// 255`). The encapsulation mode (the part before `/ip`) is folded into the suggestion's
/// description so e.g. a GRE tunnel is distinguishable from a SIT one.
///
/// Only positions that reference an *existing* tunnel should use this -- `tunnel change`,
/// `tunnel delete`, and `tunnel show`'s own optional NAME. `tunnel add`'s NAME names a tunnel
/// being created, so nothing should be suggested there.
fn parse_tunnels(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let (name, rest) = line.split_once(':')?;
            let name = name.trim();
            if name.is_empty() {
                return None;
            }
            Some(Suggestion::with_description(
                name,
                tunnel_mode(rest).unwrap_or_else(|| "IP tunnel".to_string()),
            ))
        })
        .collect_ordered_results()
}

/// Describes a tunnel from its encapsulation mode, e.g. `gre/ip` becomes `gre tunnel`.
fn tunnel_mode(rest: &str) -> Option<String> {
    let mode = rest.trim().split('/').next()?.trim();
    if mode.is_empty() {
        return None;
    }
    Some(format!("{mode} tunnel"))
}

/// Parses `ps -eo pid,comm` output (dropping the header row) into PID suggestions
/// described by the process's command name.
///
/// Used for positions that accept the PID of an arbitrary running process, such as
/// `ip netns attach NAME PID` and `ip netns identify [PID]` — these accept any
/// process on the system, not one already associated with a namespace.
fn parse_processes(output: &str) -> GeneratorResults {
    output
        .lines()
        .skip(1) // drop the "PID COMMAND" header
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let pid = parts.next()?;
            if pid.is_empty() {
                return None;
            }
            let comm = parts.collect::<Vec<_>>().join(" ");
            Some(Suggestion::with_description(
                pid,
                if comm.is_empty() {
                    "Process".to_string()
                } else {
                    comm
                },
            ))
        })
        .collect_unordered_results()
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
        .add_generator(
            "tunnel_interfaces",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr("ip tunnel show"),
                parse_tunnels,
            ),
        )
        .add_generator(
            "processes",
            Generator::script(
                CommandBuilder::single_command("ps -eo pid,comm"),
                parse_processes,
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

    /// Real `ip tunnel show` output on Linux.
    const TUNNEL_OUTPUT: &str = "gre0: gre/ip remote any local any ttl inherit nopmtudisc\ntunl1: ip/ip remote 212.93.158.1 local 212.93.129.104 ttl inherit\nsit0: ipv6/ip remote any local any ttl 64 nopmtudisc\n";

    #[test]
    fn test_parse_tunnels_keeps_the_order_ip_reports() {
        let results = parse_tunnels(TUNNEL_OUTPUT);
        let names: Vec<&str> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.exact_string.as_str())
            .collect();
        assert_eq!(names, vec!["gre0", "tunl1", "sit0"]);
        assert!(results.is_ordered);
    }

    #[test]
    fn test_parse_tunnels_describes_by_mode() {
        let results = parse_tunnels(TUNNEL_OUTPUT);
        let descriptions: Vec<Option<&str>> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.description.as_deref())
            .collect();
        assert_eq!(
            descriptions,
            vec![Some("gre tunnel"), Some("ip tunnel"), Some("ipv6 tunnel"),]
        );
    }

    #[test]
    fn test_parse_tunnels_empty_output() {
        assert!(parse_tunnels("").suggestions.is_empty());
    }

    #[test]
    fn test_parse_tunnels_skips_malformed_lines() {
        // A blank line and a line with no `:` separator (no valid tunnel name) should be
        // skipped without producing a spurious suggestion.
        let results = parse_tunnels("\ngarbage line with no colon\ngre0: gre/ip remote any local any ttl inherit nopmtudisc\n");
        let names: Vec<&str> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.exact_string.as_str())
            .collect();
        assert_eq!(names, vec!["gre0"]);
    }

    /// Real `ps -eo pid,comm` output on Linux.
    const PS_OUTPUT: &str =
        "    PID COMMAND\n      1 systemd\n    532 sshd\n    901 my long name\n";

    #[test]
    fn test_parse_processes_skips_the_header_row() {
        let results = parse_processes(PS_OUTPUT);
        let pids: Vec<&str> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.exact_string.as_str())
            .collect();
        assert_eq!(pids, vec!["1", "532", "901"]);
    }

    #[test]
    fn test_parse_processes_describes_by_command_name() {
        let results = parse_processes(PS_OUTPUT);
        let descriptions: Vec<Option<&str>> = results
            .suggestions
            .iter()
            .map(|suggestion| suggestion.description.as_deref())
            .collect();
        assert_eq!(
            descriptions,
            vec![Some("systemd"), Some("sshd"), Some("my long name")]
        );
    }

    #[test]
    fn test_parse_processes_empty_output() {
        assert!(parse_processes("").suggestions.is_empty());
    }

    #[test]
    fn test_parse_processes_header_only_output() {
        assert!(parse_processes("    PID COMMAND\n").suggestions.is_empty());
    }
}
