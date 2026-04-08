use serde::Deserialize;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandBuilder, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
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
