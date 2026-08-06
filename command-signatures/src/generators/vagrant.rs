use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Lists the machine directories Vagrant keeps for the project enclosing the working
/// directory, as `./<machine>` followed by `./<machine>/<provider>`.
///
/// Walking up for `.vagrant/machines` is how Vagrant itself locates the project root, and
/// reading that directory avoids paying `vagrant status`'s multi-second Ruby start-up on
/// every keystroke. The provider level is what [`parse_machines`] turns into the
/// suggestion's description.
const MACHINES_COMMAND: &str = "sh -c 'dir=$PWD; while [ -n \"$dir\" ]; do if [ -d \"$dir/.vagrant/machines\" ]; then cd \"$dir/.vagrant/machines\" && find . -mindepth 1 -maxdepth 2 -type d; break; fi; dir=${dir%/*}; done'";

/// Lists the box directories under the Vagrant home, honoring `$VAGRANT_HOME`.
///
/// The resolved path is quoted so a home directory containing spaces stays a single
/// argument to `ls` instead of being split into several.
const BOXES_COMMAND: &str = "sh -c 'ls -1 \"${VAGRANT_HOME:-$HOME/.vagrant.d}/boxes\"'";

/// Parses the two-level `.vagrant/machines` listing into machine-name suggestions,
/// described by the provider directory nested under each machine when there is one.
pub(super) fn parse_machines(output: &str) -> GeneratorResults {
    let mut machines: Vec<(&str, Option<&str>)> = Vec::new();
    for entry in output
        .lines()
        .filter_map(|line| line.trim().strip_prefix("./"))
        .filter(|entry| !entry.is_empty())
    {
        match entry.split_once('/') {
            None => {
                if !machines.iter().any(|(name, _)| *name == entry) {
                    machines.push((entry, None));
                }
            }
            Some((name, provider)) => {
                if let Some((_, existing)) = machines.iter_mut().find(|(m, _)| *m == name) {
                    existing.get_or_insert(provider);
                }
            }
        }
    }
    machines
        .into_iter()
        .map(|(name, provider)| match provider {
            Some(provider) => Suggestion::with_description(name, provider),
            None => Suggestion::new(name),
        })
        .collect_unordered_results()
}

/// Parses the box directory listing into box-name suggestions.
///
/// Vagrant escapes the characters that cannot appear in a directory name when it stores a
/// box, so the placeholders have to be reversed to recover the name the CLI accepts. The
/// colon is un-escaped before the slash, mirroring the order Vagrant's own
/// `BoxCollection#undir_name` uses.
pub(super) fn parse_boxes(output: &str) -> GeneratorResults {
    output
        .lines()
        .map(str::trim)
        .filter(|directory| !directory.is_empty())
        .map(|directory| {
            let name = directory
                .replace("-VAGRANTCOLON-", ":")
                .replace("-VAGRANTSLASH-", "/");
            Suggestion::with_description(name, "Installed box")
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vagrant")
        .add_generator(
            "vagrant_machines",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr(MACHINES_COMMAND),
                parse_machines,
            ),
        )
        .add_generator(
            "vagrant_boxes",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr(BOXES_COMMAND),
                parse_boxes,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let results = parse_boxes(
            "http-VAGRANTCOLON--VAGRANTSLASH--VAGRANTSLASH-example.com-VAGRANTSLASH-box\n",
        );
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

    #[test]
    fn test_boxes_command_quotes_the_resolved_path() {
        assert!(BOXES_COMMAND.contains("\"${VAGRANT_HOME:-$HOME/.vagrant.d}/boxes\""));
    }

    /// Runs the real listings against on-disk fixtures whose paths contain spaces, which
    /// an unquoted expansion would split into separate arguments.
    #[cfg(unix)]
    mod posix_execution {
        use super::*;
        use std::fs;
        use std::path::{Path, PathBuf};
        use std::process::Command;

        fn run(command: &str, working_directory: &Path, vagrant_home: &Path) -> String {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(working_directory)
                .env("VAGRANT_HOME", vagrant_home)
                .output()
                .expect("the fixture command should run");
            String::from_utf8(output.stdout).expect("the listing should be utf-8")
        }

        fn fixture_root(name: &str) -> PathBuf {
            let directory = std::env::temp_dir().join(format!("vagrant generator {name}"));
            let _ = fs::remove_dir_all(&directory);
            fs::create_dir_all(&directory).expect("the fixture directory should be creatable");
            directory
        }

        #[test]
        fn test_boxes_listing_survives_a_vagrant_home_containing_spaces() {
            let root = fixture_root("boxes");
            let vagrant_home = root.join("my vagrant home");
            let boxes = vagrant_home.join("boxes");
            fs::create_dir_all(boxes.join("hashicorp-VAGRANTSLASH-bionic64")).unwrap();
            fs::create_dir_all(boxes.join("mybox")).unwrap();

            let results = parse_boxes(&run(BOXES_COMMAND, &root, &vagrant_home));
            let mut names: Vec<&str> = results
                .suggestions
                .iter()
                .map(|s| s.exact_string.as_str())
                .collect();
            names.sort_unstable();
            assert_eq!(names, vec!["hashicorp/bionic64", "mybox"]);

            fs::remove_dir_all(&root).unwrap();
        }

        #[test]
        fn test_machines_listing_walks_up_from_a_directory_containing_spaces() {
            let root = fixture_root("machines");
            let project = root.join("my project");
            let machines = project.join(".vagrant").join("machines");
            fs::create_dir_all(machines.join("default").join("virtualbox")).unwrap();
            let nested = project.join("a nested").join("sub dir");
            fs::create_dir_all(&nested).unwrap();

            let results =
                parse_machines(&run(MACHINES_COMMAND, &nested, Path::new("/nonexistent")));
            assert_eq!(results.suggestions.len(), 1);
            assert_eq!(results.suggestions[0].exact_string, "default");
            assert_eq!(
                results.suggestions[0].description.as_deref(),
                Some("virtualbox")
            );

            fs::remove_dir_all(&root).unwrap();
        }

        #[test]
        fn test_listings_are_silent_when_there_is_nothing_to_list() {
            let root = fixture_root("empty");
            let missing = Path::new("/nonexistent");

            assert!(parse_machines(&run(MACHINES_COMMAND, &root, missing))
                .suggestions
                .is_empty());
            assert!(parse_boxes(&run(BOXES_COMMAND, &root, missing))
                .suggestions
                .is_empty());

            fs::remove_dir_all(&root).unwrap();
        }
    }
}
