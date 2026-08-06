use super::vagrant::{parse_boxes, parse_machines};

/// The subcommands `vagrant` exposes, per `vagrant list-commands`.
#[cfg(feature = "embed-signatures")]
const TOP_LEVEL_SUBCOMMANDS: [&str; 36] = [
    "autocomplete",
    "box",
    "cloud",
    "destroy",
    "docker-exec",
    "docker-logs",
    "docker-run",
    "global-status",
    "halt",
    "help",
    "init",
    "list-commands",
    "login",
    "package",
    "plugin",
    "port",
    "powershell",
    "provider",
    "provision",
    "push",
    "rdp",
    "reload",
    "resume",
    "rsync",
    "rsync-auto",
    "snapshot",
    "ssh",
    "ssh-config",
    "status",
    "suspend",
    "up",
    "upload",
    "validate",
    "version",
    "winrm",
    "winrm-config",
];

/// The command groups whose own subcommands the spec has to keep completing.
#[cfg(feature = "embed-signatures")]
const NESTED_GROUPS: [(&str, &[&str]); 4] = [
    (
        "box",
        &[
            "add",
            "help",
            "list",
            "outdated",
            "prune",
            "remove",
            "repackage",
            "update",
        ],
    ),
    (
        "cloud",
        &["auth", "box", "provider", "publish", "search", "version"],
    ),
    (
        "plugin",
        &[
            "expunge",
            "install",
            "license",
            "list",
            "repair",
            "uninstall",
            "update",
        ],
    ),
    (
        "snapshot",
        &["delete", "list", "pop", "push", "restore", "save"],
    ),
];

#[cfg(feature = "embed-signatures")]
#[test]
fn test_vagrant_spec_covers_every_subcommand_and_nested_group() {
    let vagrant = crate::signature_by_name("vagrant").expect("vagrant spec should be bundled");

    let subcommands: Vec<&str> = vagrant
        .subcommands()
        .iter()
        .map(|subcommand| subcommand.name.as_str())
        .collect();
    for name in TOP_LEVEL_SUBCOMMANDS {
        assert!(
            subcommands.contains(&name),
            "`vagrant {name}` is missing from the spec"
        );
    }

    for (group, expected) in NESTED_GROUPS {
        let signature = vagrant
            .subcommands()
            .iter()
            .find(|subcommand| subcommand.name == group)
            .unwrap_or_else(|| panic!("`vagrant {group}` is missing from the spec"));
        let nested: Vec<&str> = signature
            .subcommands()
            .iter()
            .map(|subcommand| subcommand.name.as_str())
            .collect();
        for name in expected {
            assert!(
                nested.contains(name),
                "`vagrant {group} {name}` is missing from the spec"
            );
        }
    }
}

#[test]
fn test_parse_machines_describes_each_machine_by_its_provider() {
    let output = "./default\n./default/virtualbox\n./web\n./web/libvirt\n";
    let results = parse_machines(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(names, vec!["default", "web"]);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("virtualbox")
    );
    assert_eq!(
        results.suggestions[1].description.as_deref(),
        Some("libvirt")
    );
}

#[test]
fn test_parse_machines_without_a_provider_directory() {
    let results = parse_machines("./default\n");
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "default");
    assert_eq!(results.suggestions[0].description, None);
}

#[test]
fn test_parse_machines_keeps_the_first_provider_of_a_multi_provider_machine() {
    let output = "./default\n./default/virtualbox\n./default/libvirt\n";
    let results = parse_machines(output);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("virtualbox")
    );
}

#[test]
fn test_parse_machines_skips_lines_that_are_not_listing_entries() {
    let output = "find: .: Permission denied\n.\n./default\n\n";
    let results = parse_machines(output);
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "default");
}

#[test]
fn test_parse_machines_empty_output() {
    assert!(parse_machines("").suggestions.is_empty());
}

#[test]
fn test_parse_boxes_restores_escaped_characters_in_box_names() {
    let output = "hashicorp-VAGRANTSLASH-bionic64\ngeneric-VAGRANTSLASH-ubuntu2204\nmybox\n";
    let results = parse_boxes(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|s| s.exact_string.as_str())
        .collect();
    assert_eq!(
        names,
        vec!["hashicorp/bionic64", "generic/ubuntu2204", "mybox"]
    );
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("Installed box")
    );
}

#[test]
fn test_parse_boxes_restores_escaped_colons() {
    let results =
        parse_boxes("http-VAGRANTCOLON--VAGRANTSLASH--VAGRANTSLASH-example.com-VAGRANTSLASH-box\n");
    assert_eq!(
        results.suggestions[0].exact_string,
        "http://example.com/box"
    );
}

#[test]
fn test_parse_boxes_skips_blank_lines() {
    let results = parse_boxes("\n  \nmybox\n\n");
    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "mybox");
}

#[test]
fn test_parse_boxes_empty_output() {
    assert!(parse_boxes("").suggestions.is_empty());
}
