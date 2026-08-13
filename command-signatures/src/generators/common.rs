use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use warp_completion_metadata::{
    CommandBuilder, Generator, GeneratorResults, GeneratorResultsCollector, Priority, Suggestion,
};

/// Shell command that reads ~/.ssh/config and all files referenced by Include directives.
/// Include paths are resolved by replacing ~ with $HOME and treating relative paths as
/// relative to ~/.ssh/. Glob patterns in Include paths are expanded by the shell.
pub const SSH_CONFIG_CMD: &str = "cat ~/.ssh/config $(awk 'tolower($1)==\"include\"{for(i=2;i<=NF;i++){gsub(\"~\",ENVIRON[\"HOME\"],$i);if($i!~/^\\//)$i=ENVIRON[\"HOME\"]\"/.ssh/\"$i;print $i}}' ~/.ssh/config 2>/dev/null) 2>/dev/null";

/// Parses SSH config output to extract Host entries as suggestions.
pub fn ssh_hosts(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            if line.trim().starts_with("Host ") && !line.contains('*') {
                line.split_whitespace()
                    .next_back()
                    .map(|name| Suggestion::with_description(name, "SSH Host"))
            } else {
                None
            }
        })
        .collect_unordered_results()
}

/// Returns a generator that lists SSH hosts from ~/.ssh/config (including Included files).
pub fn ssh_hosts_generator() -> Generator {
    Generator::script(CommandBuilder::single_command(SSH_CONFIG_CMD), ssh_hosts)
}

/// Helper struct used for deserializing a package.json file into the necessary fields
/// needed for generators shared across npm, yarn, pnpm, and bun.
#[derive(Deserialize)]
pub struct PackageJsonInfo {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    #[serde(default, alias = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,

    #[serde(default, alias = "optionalDependencies")]
    pub optional_dependencies: HashMap<String, String>,

    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

/// Returns a generator that lists scripts from the nearest package.json.
/// Shared across npm, yarn, pnpm, and bun.
pub fn get_scripts_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        ),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let package_info: serde_json::Result<PackageJsonInfo> = serde_json::from_str(output);

            if let Ok(package_info) = package_info {
                package_info
                    .scripts
                    .into_iter()
                    .map(|(key, value)| Suggestion::with_description(key, value))
                    .collect_unordered_results()
            } else {
                GeneratorResults::default()
            }
        },
    )
}

/// Returns a generator that lists dependencies from the nearest package.json.
/// Shared across pnpm, bun, and other package managers.
pub fn dependencies_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
        ),
        |output| {
            if output.trim().is_empty() {
                return GeneratorResults::default();
            }

            let package_info: serde_json::Result<PackageJsonInfo> = serde_json::from_str(output);
            let package_info = match package_info {
                Err(_) => return GeneratorResults::default(),
                Ok(package_info) => package_info,
            };

            let mut suggestions = package_info
                .dependencies
                .into_keys()
                .map(|key| Suggestion::with_description(key, "dependency"))
                .collect::<Vec<Suggestion>>();

            suggestions.extend(
                package_info
                    .dev_dependencies
                    .into_keys()
                    .map(|key| Suggestion::with_description(key, "devDependency")),
            );

            suggestions.extend(
                package_info
                    .optional_dependencies
                    .into_keys()
                    .map(|key| Suggestion::with_description(key, "optionalDependency")),
            );
            suggestions.into_iter().collect_unordered_results()
        },
    )
}

/// Builds the command listing systemd unit names for the system or user manager.
///
/// Loaded units and installed unit files are merged so units that are not currently
/// in memory are still offered; `awk` keeps the first line seen for each unit name.
fn systemd_units_command(user_scope: bool) -> CommandBuilder {
    let scope = if user_scope { " --user" } else { "" };
    CommandBuilder::pipe(
        CommandBuilder::single_command(format!(
            "{{ systemctl{scope} list-units --full --no-legend --no-pager --plain --all; systemctl{scope} list-unit-files --full --no-legend --no-pager --plain --all; }}"
        )),
        CommandBuilder::single_command("awk '!seen[$1]++ { print }'"),
    )
}

/// Parses `systemctl list-units` / `list-unit-files` output into unit-name suggestions,
/// described by the state column when one is present.
pub fn systemd_units(output: &str) -> GeneratorResults {
    let mut seen = HashSet::new();
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next()?;
            if name.is_empty() || !seen.insert(name.to_string()) {
                return None;
            }
            match parts.next() {
                Some(state) => Some(Suggestion::with_description(name, state)),
                None => Some(Suggestion::new(name)),
            }
        })
        .collect_unordered_results()
}

/// Returns a generator that lists units known to the system service manager.
pub fn systemd_units_generator() -> Generator {
    Generator::script(systemd_units_command(false), systemd_units)
}

/// Returns a generator that lists units known to the calling user's service manager.
pub fn systemd_user_units_generator() -> Generator {
    Generator::script(systemd_units_command(true), systemd_units)
}

/// Parses `ps -o comm` output into suggestions naming the running executables.
///
/// macOS reports absolute executable paths where Linux reports bare names, so each
/// line is reduced to its basename, which is what process-name matching expects.
/// A header row is dropped for the `ps` implementations that print one even when
/// the `comm=` format asks for none.
pub fn process_names(output: &str) -> GeneratorResults {
    let mut seen = HashSet::new();
    output
        .lines()
        .filter_map(|line| {
            let path = line.trim();
            if path.is_empty() || path == "COMM" || path == "COMMAND" {
                return None;
            }
            let name = path.rsplit_once('/').map_or(path, |(_, name)| name);
            if name.is_empty() || !seen.insert(name.to_string()) {
                return None;
            }
            Some(if name == path {
                Suggestion::new(name)
            } else {
                Suggestion::with_description(name, path)
            })
        })
        .collect_unordered_results()
}

/// Returns a cross-platform generator that lists the names of running processes.
///
/// Shared by the commands that select processes by name, such as `pkill` and `killall`.
pub fn process_names_generator() -> Generator {
    Generator::script(
        CommandBuilder::pipe(
            CommandBuilder::single_command("ps -A -o comm="),
            CommandBuilder::single_command("sort -u"),
        ),
        process_names,
    )
}

/// Parses `kill -l` output into signal-name suggestions.
pub fn signal_names(output: &str) -> GeneratorResults {
    SIGNAL_NAME
        .find_iter(output)
        .map(|capture| Suggestion::new(capture.as_str()))
        .collect_unordered_results()
}

/// Returns a generator that lists the signal names accepted by the shell's `kill`.
///
/// Shared by the commands that take a signal, such as `kill` and `pkill`.
pub fn signal_names_generator() -> Generator {
    Generator::script(CommandBuilder::single_command("env kill -l"), signal_names)
}

/// Returns a cross-platform generator that lists local user names.
///
/// Uses `getent passwd` on Linux, `dscl` on macOS, and falls back to `/etc/passwd`.
pub fn users_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "sh -c 'if command -v getent >/dev/null 2>&1; then getent passwd | cut -d: -f1; elif command -v dscl >/dev/null 2>&1; then dscl . -list /Users; else cut -d: -f1 /etc/passwd; fi'",
        ),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| {
                    !line.is_empty() && !line.starts_with('_') && !line.starts_with('#')
                })
                .map(|name| Suggestion::with_description(name.trim(), "User"))
                .collect_unordered_results()
        },
    )
}

lazy_static! {
    static ref SIGNAL_NAME: Regex = Regex::new(r"(\w+)").unwrap();
}

/// Returns a generator that lists installed pacman packages (`pacman -Q`).
///
/// Shared by `pacman` and the AUR helpers that wrap it for installed-package queries, such as
/// `yay` and `paru`.
pub fn pacman_installed_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::pipe(
            CommandBuilder::single_command("pacman -Q"),
            CommandBuilder::single_command("awk '{print $1}'"),
        ),
        |output| {
            output
                .lines()
                .map(|package_name| Suggestion::with_description(package_name, "package"))
                .collect_unordered_results()
        },
    )
}

/// Returns a generator that lists pacman package archive files (`*.pkg.tar*`) in the current
/// directory, for completing the local package file target of `-U`/`--upgrade`.
///
/// Shared by `pacman`, `yay`, and `paru`, which all accept the same archive formats.
pub fn pacman_pkg_tar_files_in_cwd_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            r#"find . -maxdepth 1 -type f -name '*.pkg.tar' -o -name '*.pkg.tar.zst' -o -name '*.pkg.tar.gz' -o -name '*.pkg.tar.xz'"#,
        ),
        |output| {
            // We should prioritize .pkg.tar files over the already installed packages.
            output
                .lines()
                .filter(|file| !file.is_empty())
                .map(|file| {
                    Suggestion::with_description(file, ".pkg.tar file")
                        .with_priority(Priority::most_important())
                })
                .collect_unordered_results()
        },
    )
}
