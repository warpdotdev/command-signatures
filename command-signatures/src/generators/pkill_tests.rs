use warp_completion_metadata::{ArgumentType, DynamicCompletionData};

use super::common::process_names;

/// Without a `pkill` signature Warp falls back to completing filesystem paths, so the
/// pattern argument must be driven by the process-name generator instead of a template.
#[cfg(feature = "embed-signatures")]
#[test]
fn test_pkill_pattern_argument_completes_process_names() {
    let signature = crate::signature_by_name("pkill").expect("pkill signature should be bundled");
    let pattern = signature
        .arguments()
        .first()
        .expect("pkill should accept a positional pattern argument");

    assert!(
        pattern.argument_types.iter().any(|argument_type| matches!(
            argument_type,
            ArgumentType::Generator(name) if name.0 == "process_name"
        )),
        "pkill's pattern argument should use the process_name generator, got {:?}",
        pattern.argument_types
    );
    assert!(
        !pattern
            .argument_types
            .iter()
            .any(|argument_type| matches!(argument_type, ArgumentType::Template(_))),
        "pkill's pattern argument should not offer file path completions, got {:?}",
        pattern.argument_types
    );
}

#[test]
fn test_pkill_registers_the_generators_its_spec_references() {
    let (command, data): (String, DynamicCompletionData) = super::pkill::generator().into();
    let names: Vec<&str> = data
        .generators()
        .keys()
        .map(|name| name.0.as_str())
        .collect();

    assert_eq!(command, "pkill");
    for expected in ["process_name", "signal_name", "user_name"] {
        assert!(
            names.contains(&expected),
            "pkill should register the {expected} generator, got {names:?}"
        );
    }
}

#[test]
fn test_process_names_uses_basenames_of_macos_style_paths() {
    let output =
        "/Applications/Warp.app/Contents/MacOS/stable\n/usr/sbin/cfprefsd\n/sbin/launchd\n";
    let results = process_names(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();

    assert_eq!(names, vec!["stable", "cfprefsd", "launchd"]);
    assert_eq!(
        results.suggestions[2].description.as_deref(),
        Some("/sbin/launchd")
    );
}

#[test]
fn test_process_names_keeps_linux_style_bare_names() {
    let output = "bash\nsystemd\nsshd\n";
    let results = process_names(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();

    assert_eq!(names, vec!["bash", "systemd", "sshd"]);
    assert_eq!(results.suggestions[0].description, None);
}

#[test]
fn test_process_names_skips_headers_blank_lines_and_duplicate_names() {
    let output = "COMMAND\nCOMM\n\n   \nbash\n/bin/bash\n/usr/bin/\n";
    let results = process_names(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();

    assert_eq!(names, vec!["bash"]);
}

#[test]
fn test_process_names_empty_output() {
    assert!(process_names("").suggestions.is_empty());
}
