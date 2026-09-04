//! Shared metadata validation for handwritten command specs.
//!
//! These checks operate on [`fig_types::Command`] before it is converted into [`crate::Signature`],
//! because conversion loses the source option boundaries (and, for persistent options, copies
//! options into descendant nodes) that precise diagnostics depend on.
//!
//! The recursive command/option traversal here is intentionally reusable: additional rules over
//! the same authored command tree (such as duplicate full option names) can be added alongside
//! [`find_short_flag_conflicts`] without re-implementing traversal.

use crate::fig_types::{html_unescape, Command, CommandOption};
use crate::signature::is_short_hand_flag;
use std::collections::{BTreeMap, HashSet};

/// A single option entry that claims a short flag involved in a [`ShortFlagConflict`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortFlagClaimant {
    /// The option's zero-based position within its command node's authored `options` array.
    pub index: usize,
    /// The complete, raw `name` array exactly as authored in the JSON (before HTML-entity
    /// unescaping).
    pub names: Vec<String>,
}

/// A short flag claimed by two or more distinct option entries directly authored on the same
/// command or subcommand node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortFlagConflict {
    /// The canonical command path, using the first declared name at each command level.
    pub command_path: Vec<String>,
    /// The normalized short flag (e.g. `-h`), after HTML-entity unescaping.
    pub flag: String,
    /// Every distinct conflicting option, in source order.
    pub claimants: Vec<ShortFlagClaimant>,
}

impl ShortFlagConflict {
    /// Formats a deterministic, human-readable diagnostic for this conflict.
    ///
    /// `file` is the (repository-relative) path of the JSON file the conflict was found in; it is
    /// supplied by the caller because `fig_types::Command` does not know its own source file.
    pub fn describe(&self, file: &str) -> String {
        let path = self.command_path.join(" ");
        let options = self
            .claimants
            .iter()
            .map(|claimant| {
                format!(
                    "#{} {}",
                    claimant.index + 1,
                    format_name_array(&claimant.names)
                )
            })
            .collect::<Vec<_>>();
        format!(
            "{file}: command \"{path}\": duplicate short flag \"{flag}\" is used by options {options}",
            file = file,
            path = path,
            flag = self.flag,
            options = join_with_and(&options),
        )
    }
}

fn format_name_array(names: &[String]) -> String {
    let entries = names
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{entries}]")
}

/// Joins strings such that `["a"]` -> `"a"`, `["a", "b"]` -> `"a and b"`, and
/// `["a", "b", "c"]` -> `"a, b, and c"`.
fn join_with_and(parts: &[String]) -> String {
    match parts {
        [] => String::new(),
        [only] => only.clone(),
        [first, second] => format!("{first} and {second}"),
        _ => {
            let (last, rest) = parts.split_last().expect("parts has more than 2 elements");
            format!("{}, and {}", rest.join(", "), last)
        }
    }
}

/// Recursively finds every duplicate-short-flag conflict directly authored on `command` and its
/// subcommands.
///
/// Each command or subcommand node is treated as an independent option namespace: sibling
/// subcommands, and a parent and its children, may reuse the same short flag without conflict.
/// Only options directly declared on a node are considered; persistent options are not yet copied
/// into descendants at this stage, so they are not double-counted as new authored definitions.
pub fn find_short_flag_conflicts(command: &Command) -> Vec<ShortFlagConflict> {
    let mut conflicts = Vec::new();
    walk(command, &[], &mut conflicts);
    conflicts
}

fn walk(command: &Command, path_prefix: &[String], conflicts: &mut Vec<ShortFlagConflict>) {
    let mut path = path_prefix.to_vec();
    // A command/subcommand's `name` array holds aliases for one namespace; the first declared
    // name is used as the canonical path segment.
    path.push(command.name.first().cloned().unwrap_or_default());

    conflicts.extend(conflicts_in_options(&command.options, &path));

    for subcommand in &command.subcommands {
        walk(subcommand, &path, conflicts);
    }
}

fn conflicts_in_options(options: &[CommandOption], path: &[String]) -> Vec<ShortFlagConflict> {
    let mut claimants_by_flag: BTreeMap<String, Vec<ShortFlagClaimant>> = BTreeMap::new();

    for (index, option) in options.iter().enumerate() {
        // Repeated spellings of the same normalized flag within one option's `name` array
        // describe one option, so it must only be counted once per flag here.
        let mut flags_claimed_by_this_option = HashSet::new();
        for name in &option.name {
            let normalized = html_unescape(name.clone());
            if is_short_hand_flag(&normalized)
                && flags_claimed_by_this_option.insert(normalized.clone())
            {
                claimants_by_flag
                    .entry(normalized)
                    .or_default()
                    .push(ShortFlagClaimant {
                        index,
                        names: option.name.clone(),
                    });
            }
        }
    }

    claimants_by_flag
        .into_iter()
        .filter(|(_, claimants)| claimants.len() > 1)
        .map(|(flag, claimants)| ShortFlagConflict {
            command_path: path.to_vec(),
            flag,
            claimants,
        })
        .collect()
}

// Focused valid/invalid fixture coverage for this rule lives in
// `completion-metadata/tests/duplicate_short_flags.rs`, which exercises the fixtures under
// `completion-metadata/tests/fixtures/duplicate_short_flags/`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_with_and_formats_by_count() {
        assert_eq!(join_with_and(&[]), "");
        assert_eq!(join_with_and(&["a".to_string()]), "a");
        assert_eq!(
            join_with_and(&["a".to_string(), "b".to_string()]),
            "a and b"
        );
        assert_eq!(
            join_with_and(&["a".to_string(), "b".to_string(), "c".to_string()]),
            "a, b, and c"
        );
    }

    #[test]
    fn format_name_array_uses_debug_quoting() {
        assert_eq!(
            format_name_array(&["-d".to_string(), "--device-id".to_string()]),
            "[\"-d\", \"--device-id\"]"
        );
    }
}
