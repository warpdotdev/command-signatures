//! Process-level integration tests for `command-signatures list`.
//!
//! Each case asserts the exit code, complete standard output, and complete standard error, and
//! confirms standard error never contains a panic diagnostic.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_command-signatures"))
}

fn write_fixture(dir: &Path, name: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).expect("failed to create fixture");
    file.write_all(contents).expect("failed to write fixture");
    path
}

fn assert_no_panic(output: &Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked at"),
        "stderr contained a panic diagnostic: {stderr}"
    );
    assert!(
        !stderr.contains("index out of bounds"),
        "stderr contained an index-out-of-bounds diagnostic: {stderr}"
    );
}

fn run_list(file: &Path, extra_args: &[&str]) -> Output {
    let output = bin()
        .arg("list")
        .arg("--file")
        .arg(file)
        .args(extra_args)
        .output()
        .expect("failed to run command-signatures list");
    assert_no_panic(&output);
    output
}

#[test]
fn empty_file_is_successful_and_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "empty.json", b"");

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"No signatures found.\n");
    assert!(output.stderr.is_empty());

    let json_output = run_list(&path, &["--json"]);
    assert!(json_output.status.success());
    assert_eq!(json_output.stdout, b"[]\n");
    assert!(json_output.stderr.is_empty());
}

#[test]
fn whitespace_only_file_is_successful_and_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "whitespace.json", b"   \n\t\r\n  ");

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"No signatures found.\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn empty_array_is_successful_and_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "array.json", b"[]");

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"No signatures found.\n");

    let json_output = run_list(&path, &["--json"]);
    assert!(json_output.status.success());
    assert_eq!(json_output.stdout, b"[]\n");
}

#[test]
fn empty_object_is_successful_and_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "object.json", b"{}");

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"No signatures found.\n");

    let json_output = run_list(&path, &["--json"]);
    assert!(json_output.status.success());
    assert_eq!(json_output.stdout, b"[]\n");
}

#[test]
fn valid_object_with_no_names_is_successful_and_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "no_names.json", br#"{"name":[]}"#);

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"No signatures found.\n");
}

#[test]
fn malformed_json_fails_with_parse_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(dir.path(), "malformed.json", b"{not json}");

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with(&format!(
            "error: failed to parse signatures file '{}': ",
            path.display()
        )),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn non_empty_object_without_name_fails_with_parse_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(
        dir.path(),
        "missing_name.json",
        br#"{"description":"missing name"}"#,
    );

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("error: failed to parse signatures file"));
}

#[test]
fn nonexistent_path_fails_with_read_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("does-not-exist.json");

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with(&format!(
            "error: failed to read signatures file '{}': ",
            path.display()
        )),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn oversized_file_fails_with_exact_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    // One byte over the 10 MiB limit, kept simple (not valid JSON, since the size check happens
    // before parsing).
    let contents = vec![b'a'; 10 * 1024 * 1024 + 1];
    let path = write_fixture(dir.path(), "oversized.json", &contents);

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        format!(
            "error: signatures file '{}' exceeds maximum size of 10485760 bytes\n",
            path.display()
        )
        .into_bytes()
    );
}

#[test]
fn depth_65_fails_with_exact_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    let mut contents = vec![b'['; 65];
    contents.extend(vec![b']'; 65]);
    let path = write_fixture(dir.path(), "too_deep.json", &contents);

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        format!(
            "error: signatures file '{}' exceeds maximum JSON nesting depth of 64\n",
            path.display()
        )
        .into_bytes()
    );
}

#[test]
fn ten_thousand_and_one_commands_fails_with_exact_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    let mut contents = vec![b'['];
    for i in 0..10_001 {
        if i > 0 {
            contents.push(b',');
        }
        contents.extend(format!(r#"{{"name":"cmd{i}"}}"#).into_bytes());
    }
    contents.push(b']');
    let path = write_fixture(dir.path(), "too_many.json", &contents);

    let output = run_list(&path, &[]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        format!(
            "error: signatures file '{}' contains more than 10000 commands\n",
            path.display()
        )
        .into_bytes()
    );
}

#[test]
fn valid_single_object_and_array_produce_sorted_rows() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(
        dir.path(),
        "single.json",
        br#"{"name":"zeta","description":"last one","subcommands":[{"name":"a"},{"name":"b"}]}"#,
    );

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"NAME\tSUBCOMMANDS\tDESCRIPTION\nzeta\t2\tlast one\n".to_vec()
    );

    let json_output = run_list(&path, &["--json"]);
    assert!(json_output.status.success());
    let expected = br#"[{"name":"zeta","description":"last one","subcommand_count":2}]"#.to_vec();
    let mut expected_with_newline = expected;
    expected_with_newline.push(b'\n');
    assert_eq!(json_output.stdout, expected_with_newline);

    let array_path = write_fixture(
        dir.path(),
        "array_of_two.json",
        br#"[{"name":"beta"},{"name":"alpha","description":"first"}]"#,
    );
    let array_output = run_list(&array_path, &[]);
    assert!(array_output.status.success());
    assert_eq!(
        array_output.stdout,
        b"NAME\tSUBCOMMANDS\tDESCRIPTION\nalpha\t0\tfirst\nbeta\t0\t\n".to_vec()
    );
}

#[test]
fn control_characters_in_name_and_description_cannot_forge_columns_or_rows() {
    // A name and description containing tabs, newlines, and carriage returns (as JSON escapes)
    // must not be able to inject extra tab-separated columns or extra rows into the text output:
    // this is untrusted `--file` content, and the approved spec requires exactly one line per
    // signature.
    let dir = tempfile::tempdir().unwrap();
    let path = write_fixture(
        dir.path(),
        "control_chars.json",
        br#"{"name":"evil\tname\nwith\rcontrol","description":"desc\twith\ttabs\nand\nnewlines"}"#,
    );

    let output = run_list(&path, &[]);
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"NAME\tSUBCOMMANDS\tDESCRIPTION\nevil name with control\t0\tdesc with tabs and newlines\n"
            .to_vec()
    );
    // Exactly one header line and one data line: no extra rows or columns were injected.
    assert_eq!(output.stdout.iter().filter(|&&b| b == b'\n').count(), 2);
    let data_line = output.stdout.split(|&b| b == b'\n').nth(1).unwrap();
    assert_eq!(data_line.iter().filter(|&&b| b == b'\t').count(), 2);
}

#[test]
fn default_embedded_source_lists_repository_signatures() {
    let output = bin().arg("list").output().unwrap();
    assert!(output.status.success());
    assert_no_panic(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("NAME\tSUBCOMMANDS\tDESCRIPTION\n"));
    assert!(stdout.lines().count() > 1);

    let json_output = bin().arg("list").arg("--json").output().unwrap();
    assert!(json_output.status.success());
    let parsed: serde_json::Value = serde_json::from_slice(&json_output.stdout).unwrap();
    let array = parsed.as_array().expect("expected a JSON array");
    assert!(!array.is_empty());
}

#[test]
fn unknown_subcommand_is_a_usage_error() {
    let output = bin().arg("frobnicate").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn missing_file_value_is_a_usage_error() {
    let output = bin().arg("list").arg("--file").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
}
