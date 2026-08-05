use super::common::systemd_units;
use super::journalctl::{parse_boots, parse_fields};

#[test]
fn test_parse_boots_modern_table_output() {
    // Newer systemd versions print a header row before the boot list.
    let output = "IDX BOOT ID                          FIRST ENTRY                 LAST ENTRY\n -1 4b9b9a1b0b0d4d8fa1b2c3d4e5f60718 Mon 2024-05-06 09:12:31 UTC Mon 2024-05-06 18:44:02 UTC\n  0 8f1c2d3e4a5b6c7d8e9f0a1b2c3d4e5f Tue 2024-05-07 08:01:14 UTC Tue 2024-05-07 12:00:00 UTC\n";
    let results = parse_boots(output);
    assert_eq!(results.suggestions.len(), 2);
    assert_eq!(results.suggestions[0].exact_string, "-1");
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("4b9b9a1b0b0d4d8fa1b2c3d4e5f60718 (Mon 2024-05-06 09:12:31 UTC Mon 2024-05-06 18:44:02 UTC)")
    );
    assert_eq!(results.suggestions[1].exact_string, "0");
    assert!(results.is_ordered);
}

#[test]
fn test_parse_boots_legacy_output_without_header() {
    let output = "-1 4b9b9a1b0b0d4d8fa1b2c3d4e5f60718 Mon 2024-05-06 09:12:31 UTC—Mon 2024-05-06 18:44:02 UTC\n 0 8f1c2d3e4a5b6c7d8e9f0a1b2c3d4e5f Tue 2024-05-07 08:01:14 UTC—Tue 2024-05-07 12:00:00 UTC\n";
    let results = parse_boots(output);
    let offsets: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(offsets, vec!["-1", "0"]);
}

#[test]
fn test_parse_boots_without_timestamps_falls_back_to_boot_id() {
    let output = "0 8f1c2d3e4a5b6c7d8e9f0a1b2c3d4e5f\n";
    let results = parse_boots(output);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("8f1c2d3e4a5b6c7d8e9f0a1b2c3d4e5f")
    );
}

#[test]
fn test_parse_boots_skips_lines_without_an_offset_and_boot_id() {
    let output = "Failed to determine boots: No such file or directory\n0\n\n0 8f1c2d3e4a5b6c7d8e9f0a1b2c3d4e5f\n";
    let results = parse_boots(output);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "0");
}

#[test]
fn test_parse_boots_empty_output() {
    assert!(parse_boots("").suggestions.is_empty());
}

#[test]
fn test_parse_fields_as_option_argument() {
    let output = "MESSAGE\n_SYSTEMD_UNIT\nPRIORITY\n";
    let results = parse_fields(output, false);
    let fields: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(fields, vec!["MESSAGE", "_SYSTEMD_UNIT", "PRIORITY"]);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("Journal field")
    );
}

#[test]
fn test_parse_fields_as_match_appends_equals() {
    let output = "MESSAGE\n_SYSTEMD_UNIT\n";
    let results = parse_fields(output, true);
    let fields: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(fields, vec!["MESSAGE=", "_SYSTEMD_UNIT="]);
}

#[test]
fn test_parse_fields_skips_blank_lines() {
    let results = parse_fields("\n  \nMESSAGE\n\n", false);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "MESSAGE");
}

#[test]
fn test_systemd_units_describes_units_by_state_and_deduplicates() {
    let output = "ssh.service loaded active running OpenBSD Secure Shell server\ncron.service loaded active running Regular background program processing daemon\nssh.service enabled enabled\nnetworking.service\n";
    let results = systemd_units(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(
        names,
        vec!["ssh.service", "cron.service", "networking.service"]
    );
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("loaded")
    );
    assert_eq!(results.suggestions[2].description, None);
}

#[test]
fn test_systemd_units_empty_output() {
    assert!(systemd_units("").suggestions.is_empty());
}
