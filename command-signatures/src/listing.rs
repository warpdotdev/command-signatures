//! Reusable listing of Fig-compatible command signatures.
//!
//! This module backs the `command-signatures list` CLI subcommand, but is a public library API
//! so any Rust caller can list either the signatures embedded into this crate or an external,
//! Fig-compatible JSON document.

use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::de::{self, Deserializer as _, SeqAccess, Visitor};
use warp_completion_metadata::fig_types::Command;
use warp_completion_metadata::Signature;

/// Maximum size, in bytes, of an external signatures file that [`list_signatures`] will read.
pub const MAX_EXTERNAL_FILE_BYTES: u64 = 10 * 1024 * 1024;

/// Maximum nesting depth of JSON containers (`{}`/`[]`) accepted in an external signatures file.
/// The root object or array is depth 1; each nested container adds 1.
pub const MAX_JSON_NESTING_DEPTH: usize = 64;

/// Maximum number of command objects accepted in a top-level array in an external signatures
/// file.
pub const MAX_EXTERNAL_COMMANDS: usize = 10_000;

/// A marker used to distinguish a "too many commands" failure from an ordinary parse failure
/// when it surfaces through `serde`'s generic error type.
const TOO_MANY_COMMANDS_MARKER: &str = "warp-command-signatures: too many commands";

/// Where [`list_signatures`] should read command signatures from.
#[derive(Debug, Clone)]
pub enum SignatureSource {
    /// The signatures embedded into this binary at compile time.
    Embedded,
    /// An external, Fig-compatible JSON document at the given path. Replaces the embedded
    /// source rather than merging with it.
    File(PathBuf),
}

/// One row of [`list_signatures`] output.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SignatureSummary {
    pub name: String,
    pub description: Option<String>,
    pub subcommand_count: usize,
}

/// An error encountered while listing signatures from an external file.
///
/// Each variant retains the display path and, where applicable, the underlying error so callers
/// do not need to parse a formatted string.
#[derive(Debug)]
pub enum ListSignaturesError {
    /// The file could not be read, e.g. it did not exist, was a directory, or was not
    /// accessible.
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The file's contents were malformed JSON or did not satisfy the Fig-compatible command
    /// schema.
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    /// The file exceeded [`MAX_EXTERNAL_FILE_BYTES`].
    InputTooLarge { path: PathBuf },
    /// The file's JSON nesting exceeded [`MAX_JSON_NESTING_DEPTH`].
    NestingTooDeep { path: PathBuf },
    /// The file's top-level array contained more than [`MAX_EXTERNAL_COMMANDS`] commands.
    TooManyCommands { path: PathBuf },
}

impl fmt::Display for ListSignaturesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(
                f,
                "failed to read signatures file '{}': {source}",
                path.display()
            ),
            Self::Parse { path, source } => write!(
                f,
                "failed to parse signatures file '{}': {source}",
                path.display()
            ),
            Self::InputTooLarge { path } => write!(
                f,
                "signatures file '{}' exceeds maximum size of {MAX_EXTERNAL_FILE_BYTES} bytes",
                path.display()
            ),
            Self::NestingTooDeep { path } => write!(
                f,
                "signatures file '{}' exceeds maximum JSON nesting depth of {MAX_JSON_NESTING_DEPTH}",
                path.display()
            ),
            Self::TooManyCommands { path } => write!(
                f,
                "signatures file '{}' contains more than {MAX_EXTERNAL_COMMANDS} commands",
                path.display()
            ),
        }
    }
}

impl std::error::Error for ListSignaturesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::InputTooLarge { .. }
            | Self::NestingTooDeep { .. }
            | Self::TooManyCommands { .. } => None,
        }
    }
}

/// Lists and summarizes the command signatures from `source`, sorted deterministically.
///
/// This never executes a generator or other shell command referenced by a signature: it only
/// inspects the static schema.
pub fn list_signatures(
    source: SignatureSource,
) -> Result<Vec<SignatureSummary>, ListSignaturesError> {
    let signatures = match source {
        SignatureSource::Embedded => crate::commands(),
        SignatureSource::File(path) => read_external_signatures(&path)?,
    };
    Ok(summarize(signatures))
}

/// Converts signatures into sorted summaries. Rows are ordered case-insensitively by name, with
/// the original name as a tie-breaker, so output is deterministic even when the source order is
/// not (embedded loading is parallel).
fn summarize(signatures: Vec<Signature>) -> Vec<SignatureSummary> {
    let mut summaries: Vec<SignatureSummary> = signatures
        .iter()
        .map(|signature| SignatureSummary {
            name: signature.name().to_string(),
            description: signature.description.clone(),
            subcommand_count: signature.subcommands().len(),
        })
        .collect();
    summaries.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.name.cmp(&b.name))
    });
    summaries
}

fn is_json_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r')
}

fn is_json_whitespace_only(bytes: &[u8]) -> bool {
    bytes.iter().all(|&byte| is_json_whitespace(byte))
}

/// Trims leading and trailing JSON whitespace. The caller is responsible for handling an
/// all-whitespace (or empty) input before calling this.
fn trim_json_whitespace(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|&byte| !is_json_whitespace(byte))
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|&byte| !is_json_whitespace(byte))
        .map_or(start, |index| index + 1);
    &bytes[start..end]
}

/// Reads `path` through a bounded reader capped at [`MAX_EXTERNAL_FILE_BYTES`] + 1, so a
/// maliciously large file is never fully materialized in memory.
fn read_bounded(path: &Path) -> Result<Vec<u8>, ListSignaturesError> {
    let map_read_err = |source: std::io::Error| ListSignaturesError::Read {
        path: path.to_path_buf(),
        source,
    };

    let file = std::fs::File::open(path).map_err(map_read_err)?;
    let mut bounded = file.take(MAX_EXTERNAL_FILE_BYTES + 1);
    let mut buffer = Vec::new();
    bounded.read_to_end(&mut buffer).map_err(map_read_err)?;

    if buffer.len() as u64 > MAX_EXTERNAL_FILE_BYTES {
        return Err(ListSignaturesError::InputTooLarge {
            path: path.to_path_buf(),
        });
    }

    Ok(buffer)
}

/// Scans `bytes` once with a string- and escape-aware structural scanner to bound JSON nesting
/// depth before any recursive JSON materialization. JSON delimiters inside strings, including
/// escaped quotes and backslashes, do not contribute to the nesting count.
fn check_nesting_depth(bytes: &[u8], path: &Path) -> Result<(), ListSignaturesError> {
    let mut depth: usize = 0;
    let mut in_string = false;
    let mut escaped = false;

    for &byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth += 1;
                if depth > MAX_JSON_NESTING_DEPTH {
                    return Err(ListSignaturesError::NestingTooDeep {
                        path: path.to_path_buf(),
                    });
                }
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }

    Ok(())
}

/// An object containing only JSON whitespace between its braces, such as `{}` or `{ }`, is
/// treated as an empty collection for graceful compatibility with common empty JSON documents.
fn is_empty_object(trimmed: &[u8]) -> bool {
    trimmed.len() >= 2
        && trimmed[0] == b'{'
        && trimmed[trimmed.len() - 1] == b'}'
        && is_json_whitespace_only(&trimmed[1..trimmed.len() - 1])
}

/// Parses a non-array root: either the empty-object exception, or one `Command`. A non-object,
/// non-array root (`null`, a boolean, a number, or a string) fails the `Command` deserialization
/// naturally and is reported as a parse error.
fn parse_non_array_root(trimmed: &[u8], path: &Path) -> Result<Vec<Command>, ListSignaturesError> {
    if is_empty_object(trimmed) {
        return Ok(Vec::new());
    }

    let command: Command =
        serde_json::from_slice(trimmed).map_err(|source| ListSignaturesError::Parse {
            path: path.to_path_buf(),
            source,
        })?;
    Ok(vec![command])
}

/// A `Visitor` that deserializes a JSON array of `Command` values one element at a time,
/// probing for at most one element beyond [`MAX_EXTERNAL_COMMANDS`] rather than deserializing an
/// unbounded array and counting afterward.
struct BoundedCommandSeq;

impl<'de> Visitor<'de> for BoundedCommandSeq {
    type Value = Vec<Command>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an array of Fig-compatible command objects")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut commands = Vec::new();
        while let Some(command) = seq.next_element::<Command>()? {
            if commands.len() == MAX_EXTERNAL_COMMANDS {
                return Err(de::Error::custom(TOO_MANY_COMMANDS_MARKER));
            }
            commands.push(command);
        }
        Ok(commands)
    }
}

/// Parses an array root through the bounded sequence visitor above.
fn parse_array_root(trimmed: &[u8], path: &Path) -> Result<Vec<Command>, ListSignaturesError> {
    let mut deserializer = serde_json::Deserializer::from_slice(trimmed);
    let commands =
        deserializer
            .deserialize_seq(BoundedCommandSeq)
            .map_err(|source: serde_json::Error| {
                if source.to_string().contains(TOO_MANY_COMMANDS_MARKER) {
                    ListSignaturesError::TooManyCommands {
                        path: path.to_path_buf(),
                    }
                } else {
                    ListSignaturesError::Parse {
                        path: path.to_path_buf(),
                        source,
                    }
                }
            })?;
    // Require the parser to consume the complete document so trailing non-whitespace bytes are
    // malformed, matching the behavior `serde_json::from_slice` provides for other roots.
    deserializer
        .end()
        .map_err(|source| ListSignaturesError::Parse {
            path: path.to_path_buf(),
            source,
        })?;
    Ok(commands)
}

/// Reads, validates, and parses an external signatures file into signatures. Empty bytes,
/// whitespace-only bytes, `[]`, and `{}` are all successful, empty collections.
fn read_external_signatures(path: &Path) -> Result<Vec<Signature>, ListSignaturesError> {
    let bytes = read_bounded(path)?;

    if is_json_whitespace_only(&bytes) {
        return Ok(Vec::new());
    }

    check_nesting_depth(&bytes, path)?;

    let trimmed = trim_json_whitespace(&bytes);
    let commands = if trimmed.first() == Some(&b'[') {
        parse_array_root(trimmed, path)?
    } else {
        parse_non_array_root(trimmed, path)?
    };

    // Convert every accepted `Command` with the existing `Vec::<Signature>::from` implementation
    // used for embedded assets, so external and embedded sources share one conversion path.
    Ok(commands
        .into_iter()
        .flat_map(Vec::<Signature>::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_fixture(contents: &[u8]) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().expect("failed to create temp file");
        file.write_all(contents).expect("failed to write fixture");
        file.flush().expect("failed to flush fixture");
        file
    }

    fn list_file(contents: &[u8]) -> Result<Vec<SignatureSummary>, ListSignaturesError> {
        let file = write_fixture(contents);
        list_signatures(SignatureSource::File(file.path().to_path_buf()))
    }

    #[test]
    fn empty_bytes_are_an_empty_collection() {
        assert_eq!(list_file(b"").unwrap(), Vec::new());
    }

    #[test]
    fn whitespace_only_bytes_are_an_empty_collection() {
        assert_eq!(list_file(b"   \n\t\r\n  ").unwrap(), Vec::new());
    }

    #[test]
    fn empty_array_is_an_empty_collection() {
        assert_eq!(list_file(b"[]").unwrap(), Vec::new());
    }

    #[test]
    fn empty_object_is_an_empty_collection() {
        assert_eq!(list_file(b"{}").unwrap(), Vec::new());
        assert_eq!(list_file(b"{ \n\t }").unwrap(), Vec::new());
    }

    #[test]
    fn valid_object_with_no_names_is_an_empty_collection() {
        assert_eq!(list_file(br#"{"name":[]}"#).unwrap(), Vec::new());
    }

    #[test]
    fn single_command_object_produces_one_row() {
        let summaries = list_file(
            br#"{"name":"foo","description":"does foo things","subcommands":[{"name":"bar"}]}"#,
        )
        .unwrap();
        assert_eq!(
            summaries,
            vec![SignatureSummary {
                name: "foo".to_string(),
                description: Some("does foo things".to_string()),
                subcommand_count: 1,
            }]
        );
    }

    #[test]
    fn command_array_flattens_all_entries() {
        let summaries =
            list_file(br#"[{"name":"foo"},{"name":"bar"},{"name":["baz","qux"]}]"#).unwrap();
        let names: Vec<&str> = summaries.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["bar", "baz", "foo", "qux"]);
    }

    #[test]
    fn ordering_is_case_insensitive_with_original_name_tie_break() {
        let summaries =
            list_file(br#"[{"name":"Bravo"},{"name":"alpha"},{"name":"bravo"}]"#).unwrap();
        let names: Vec<&str> = summaries.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["alpha", "Bravo", "bravo"]);
    }

    #[test]
    fn malformed_json_is_a_parse_error() {
        let err = list_file(b"{not json}").unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));
    }

    #[test]
    fn scalar_root_is_a_parse_error() {
        for input in [&b"null"[..], b"true", b"42", b"\"a string\""] {
            let err = list_file(input).unwrap_err();
            assert!(
                matches!(err, ListSignaturesError::Parse { .. }),
                "expected a parse error for {input:?}"
            );
        }
    }

    #[test]
    fn non_empty_object_missing_name_is_a_parse_error() {
        let err = list_file(br#"{"description":"missing name"}"#).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));
    }

    #[test]
    fn array_with_invalid_member_is_a_parse_error() {
        let err = list_file(br#"[{"name":"foo"},1]"#).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));
    }

    #[test]
    fn trailing_bytes_are_a_parse_error() {
        let err = list_file(br#"{"name":"foo"} garbage"#).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));
        let err = list_file(br#"[{"name":"foo"}] garbage"#).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));
    }

    #[test]
    fn nonexistent_path_is_a_read_error() {
        let err = list_signatures(SignatureSource::File(PathBuf::from(
            "/nonexistent/path/does-not-exist.json",
        )))
        .unwrap_err();
        assert!(matches!(err, ListSignaturesError::Read { .. }));
    }

    #[test]
    fn directory_path_is_a_read_error() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let err = list_signatures(SignatureSource::File(dir.path().to_path_buf())).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Read { .. }));
    }

    #[test]
    fn exactly_max_bytes_is_accepted_but_one_more_is_too_large() {
        // A padded, otherwise-valid document: an object with a name, then padding via a long
        // description so the exact byte count can be controlled.
        let prefix = br#"{"name":"foo","description":""#;
        let suffix = br#""}"#;
        let max_len = MAX_EXTERNAL_FILE_BYTES as usize;
        let padding_len = max_len - prefix.len() - suffix.len();
        let mut exact = Vec::with_capacity(max_len);
        exact.extend_from_slice(prefix);
        exact.extend(vec![b'a'; padding_len]);
        exact.extend_from_slice(suffix);
        assert_eq!(exact.len(), max_len);
        let summaries = list_file(&exact).unwrap();
        assert_eq!(summaries.len(), 1);

        let mut too_large = exact;
        // Insert one more padding byte before the closing quote, keeping the document otherwise
        // valid JSON, so the failure is unambiguously about size and not shape.
        let insert_at = too_large.len() - suffix.len();
        too_large.insert(insert_at, b'a');
        assert_eq!(too_large.len(), max_len + 1);
        let err = list_file(&too_large).unwrap_err();
        assert!(matches!(err, ListSignaturesError::InputTooLarge { .. }));
    }

    fn nested_array_json(depth: usize) -> Vec<u8> {
        let mut json = vec![b'['; depth];
        json.extend(vec![b']'; depth]);
        json
    }

    #[test]
    fn depth_64_parses_normally_depth_65_is_too_deep() {
        // At exactly depth 64 nesting is accepted; this document is a well-formed but
        // schema-invalid array-of-arrays, so it reaches ordinary parsing and fails there instead
        // of on the depth check.
        let err = list_file(&nested_array_json(64)).unwrap_err();
        assert!(matches!(err, ListSignaturesError::Parse { .. }));

        let err = list_file(&nested_array_json(65)).unwrap_err();
        assert!(matches!(err, ListSignaturesError::NestingTooDeep { .. }));
    }

    #[test]
    fn depth_scanner_ignores_delimiters_inside_strings() {
        // A single string value containing many brackets should not trip the depth check.
        let mut json = br#"{"name":"foo","description":""#.to_vec();
        json.extend(vec![b'['; 200]);
        json.extend(vec![b']'; 200]);
        json.extend(br#""}"#);
        let summaries = list_file(&json).unwrap();
        assert_eq!(summaries.len(), 1);
    }

    fn command_array_json(count: usize) -> Vec<u8> {
        let mut json = vec![b'['];
        for i in 0..count {
            if i > 0 {
                json.push(b',');
            }
            json.extend(format!(r#"{{"name":"cmd{i}"}}"#).into_bytes());
        }
        json.push(b']');
        json
    }

    #[test]
    fn exactly_max_commands_is_accepted_but_one_more_is_too_many() {
        let summaries = list_file(&command_array_json(MAX_EXTERNAL_COMMANDS)).unwrap();
        assert_eq!(summaries.len(), MAX_EXTERNAL_COMMANDS);

        let err = list_file(&command_array_json(MAX_EXTERNAL_COMMANDS + 1)).unwrap_err();
        assert!(matches!(err, ListSignaturesError::TooManyCommands { .. }));
    }

    #[test]
    fn text_normalization_preserves_one_row_per_signature() {
        let summaries = list_file(br#"[{"name":"a"},{"name":"b"},{"name":"c"}]"#).unwrap();
        assert_eq!(summaries.len(), 3);
    }

    #[test]
    fn json_summary_serializes_with_documented_field_names() {
        let summary = SignatureSummary {
            name: "foo".to_string(),
            description: None,
            subcommand_count: 0,
        };
        let json = serde_json::to_string(&summary).unwrap();
        assert_eq!(
            json,
            r#"{"name":"foo","description":null,"subcommand_count":0}"#
        );
    }

    #[test]
    fn embedded_source_lists_the_repository_signatures() {
        let summaries = list_signatures(SignatureSource::Embedded).unwrap();
        assert!(!summaries.is_empty());
    }
}
