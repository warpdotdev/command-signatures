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
const MACHINES_POSIX: &str = "sh -c 'dir=$PWD; while [ -n \"$dir\" ]; do if [ -d \"$dir/.vagrant/machines\" ]; then cd \"$dir/.vagrant/machines\" && find . -mindepth 1 -maxdepth 2 -type d; break; fi; dir=${dir%/*}; done'";

/// The PowerShell equivalent of [`MACHINES_POSIX`], emitting the same `./<machine>` and
/// `./<machine>/<provider>` lines so both platforms share one parser.
///
/// The machine name is captured into `$n` and the provider directories are listed in a
/// nested loop, rather than recursing and rewriting path separators, so the script needs
/// no backslash literals and survives being passed through `cmd.exe`.
const MACHINES_POWERSHELL: &str = "$d = $PWD.Path; while ($d) { $m = Join-Path $d '.vagrant/machines'; if (Test-Path -LiteralPath $m) { Get-ChildItem -LiteralPath $m -Directory | ForEach-Object { $n = $_.Name; './' + $n; Get-ChildItem -LiteralPath $_.FullName -Directory | ForEach-Object { './' + $n + '/' + $_.Name } }; break }; $d = Split-Path -Parent $d }";

/// Lists the box directories under the Vagrant home, honoring `$VAGRANT_HOME`.
///
/// The resolved path is quoted so a home directory containing spaces stays a single
/// argument to `ls` instead of being split into several.
const BOXES_POSIX: &str = "sh -c 'ls -1 \"${VAGRANT_HOME:-$HOME/.vagrant.d}/boxes\"'";

/// The PowerShell equivalent of [`BOXES_POSIX`].
const BOXES_POWERSHELL: &str = "$h = $env:VAGRANT_HOME; if (-not $h) { $h = Join-Path $HOME '.vagrant.d' }; Get-ChildItem -LiteralPath (Join-Path $h 'boxes') -Directory -Name";

/// Wraps a PowerShell script so it can be launched from `cmd.exe`.
///
/// `cmd.exe` leaves `$` alone and expands only `%VAR%`, so double-quoting the script keeps
/// it intact as long as the script itself quotes with `'`.
fn powershell_from_cmd_exe(script: &str) -> String {
    format!("powershell -NoProfile -Command \"{script}\"")
}

/// Builds the shell-appropriate command for one of the two directory listings.
fn per_shell_command(posix: &str, powershell: &str) -> CommandBuilder {
    CommandBuilder::per_shell_and_ignore_stderr(
        posix,
        powershell,
        powershell_from_cmd_exe(powershell),
    )
}

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
                per_shell_command(MACHINES_POSIX, MACHINES_POWERSHELL),
                parse_machines,
            ),
        )
        .add_generator(
            "vagrant_boxes",
            Generator::script(
                per_shell_command(BOXES_POSIX, BOXES_POWERSHELL),
                parse_boxes,
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp_completion_metadata::Shell;

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
    fn test_machines_command_is_selected_per_shell() {
        let command = per_shell_command(MACHINES_POSIX, MACHINES_POWERSHELL);

        let posix = command.build(Shell::Posix);
        assert!(posix.starts_with("sh -c "));
        assert!(posix.contains("find . -mindepth 1 -maxdepth 2 -type d"));

        let powershell = command.build(Shell::Powershell);
        assert!(powershell.starts_with("$d = $PWD.Path"));
        assert!(!powershell.contains("sh -c "));
        assert!(!powershell.contains("find "));

        let cmd_exe = command.build(Shell::CmdExe);
        assert!(cmd_exe.starts_with("powershell -NoProfile -Command \""));
        assert!(!cmd_exe.contains("sh -c "));
    }

    #[test]
    fn test_boxes_command_is_selected_per_shell() {
        let command = per_shell_command(BOXES_POSIX, BOXES_POWERSHELL);

        let posix = command.build(Shell::Posix);
        assert!(posix.starts_with("sh -c "));
        assert!(posix.contains("ls -1 "));

        let powershell = command.build(Shell::Powershell);
        assert!(powershell.contains("Get-ChildItem"));
        assert!(!powershell.contains("sh -c "));
        assert!(!powershell.contains("ls -1 "));

        let cmd_exe = command.build(Shell::CmdExe);
        assert!(cmd_exe.starts_with("powershell -NoProfile -Command \""));
        assert!(cmd_exe.contains("Get-ChildItem"));
    }

    /// A `cmd.exe` wrapper only stays intact while the script quotes with `'`; an embedded
    /// `"` would terminate the wrapper's own quoting and truncate the script.
    #[test]
    fn test_powershell_scripts_carry_no_double_quotes() {
        for script in [MACHINES_POWERSHELL, BOXES_POWERSHELL] {
            assert!(!script.contains('"'), "`{script}` contains a double quote");
        }
    }

    #[test]
    fn test_boxes_posix_command_quotes_the_resolved_path() {
        assert!(BOXES_POSIX.contains("\"${VAGRANT_HOME:-$HOME/.vagrant.d}/boxes\""));
    }

    /// Runs the real POSIX listings against on-disk fixtures whose paths contain spaces,
    /// which an unquoted expansion would split into separate arguments.
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

            let results = parse_boxes(&run(BOXES_POSIX, &root, &vagrant_home));
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

            let results = parse_machines(&run(MACHINES_POSIX, &nested, Path::new("/nonexistent")));
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

            assert!(parse_machines(&run(MACHINES_POSIX, &root, missing))
                .suggestions
                .is_empty());
            assert!(parse_boxes(&run(BOXES_POSIX, &root, missing))
                .suggestions
                .is_empty());

            fs::remove_dir_all(&root).unwrap();
        }
    }
}
