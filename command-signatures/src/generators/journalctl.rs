use super::common;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Parses `journalctl --list-boots` output into boot-offset suggestions.
///
/// The offset (`0` for the current boot, `-1` for the previous one, …) is what `--boot`
/// takes most often, so it is the suggestion and the boot ID plus timestamps become its
/// description. Lines whose first column is not an offset — such as the table header
/// printed by newer systemd versions — are skipped.
pub(super) fn parse_boots(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let offset = parts.next()?;
            offset.parse::<i64>().ok()?;
            let boot_id = parts.next()?;
            let timestamps = parts.collect::<Vec<_>>().join(" ");
            let description = if timestamps.is_empty() {
                boot_id.to_string()
            } else {
                format!("{boot_id} ({timestamps})")
            };
            Some(Suggestion::with_description(offset, description))
        })
        .collect_ordered_results()
}

/// Parses `journalctl --fields` output into journal field-name suggestions, appending
/// `=` when the field is being completed as a `FIELD=VALUE` match rather than as the
/// argument of an option such as `--field`.
pub(super) fn parse_fields(output: &str, as_match: bool) -> GeneratorResults {
    output
        .lines()
        .map(str::trim)
        .filter(|field| !field.is_empty())
        .map(|field| {
            let value = if as_match {
                format!("{field}=")
            } else {
                field.to_string()
            };
            Suggestion::with_description(value, "Journal field")
        })
        .collect_unordered_results()
}

/// Builds a `journalctl` query that reports only its own results, so a missing binary or
/// an unreadable journal yields no suggestions instead of error text.
fn journalctl_query(args: &str) -> CommandBuilder {
    CommandBuilder::single_command_and_ignore_stderr(format!(
        "journalctl --no-pager --quiet {args}"
    ))
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("journalctl")
        .add_generator("units", common::systemd_units_generator())
        .add_generator("user_units", common::systemd_user_units_generator())
        .add_generator(
            "boots",
            Generator::script(journalctl_query("--list-boots"), parse_boots),
        )
        .add_generator(
            "journal_fields",
            Generator::script(journalctl_query("--fields"), |output| {
                parse_fields(output, false)
            }),
        )
        .add_generator(
            "journal_field_matches",
            Generator::script(journalctl_query("--fields"), |output| {
                parse_fields(output, true)
            }),
        )
        .add_generator(
            "syslog_identifiers",
            Generator::script(journalctl_query("--field SYSLOG_IDENTIFIER"), |output| {
                output
                    .lines()
                    .map(str::trim)
                    .filter(|identifier| !identifier.is_empty())
                    .map(|identifier| Suggestion::with_description(identifier, "Syslog identifier"))
                    .collect_unordered_results()
            }),
        )
}
