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
pub(super) fn parse_interfaces(output: &str) -> GeneratorResults {
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
