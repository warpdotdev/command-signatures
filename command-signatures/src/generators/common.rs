use serde::Deserialize;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandBuilder, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

/// Shell command that reads ~/.ssh/config and all files referenced by Include directives.
/// Include paths are resolved by replacing ~ with $HOME, normalizing Windows drive-letter
/// paths (`C:\Users\..` / `C:/Users/..` -> `/c/Users/..`) so they resolve under Git Bash /
/// MSYS, and treating other relative paths as relative to ~/.ssh/. Glob patterns in Include
/// paths are expanded by the shell. Only rooted drive paths (`C:\..`, `C:/..`) are normalized;
/// drive-relative (`C:foo`) and UNC (`\\server\share`) forms are not (ssh does not accept them
/// in Include either).
pub const SSH_CONFIG_CMD: &str = "cat ~/.ssh/config $(awk 'tolower($1)==\"include\"{for(i=2;i<=NF;i++){gsub(\"~\",ENVIRON[\"HOME\"],$i);if($i~/^[A-Za-z]:/){d=tolower(substr($i,1,1));r=substr($i,3);gsub(/\\\\/,\"/\",r);$i=\"/\"d r}else if($i!~/^\\//)$i=ENVIRON[\"HOME\"]\"/.ssh/\"$i;print $i}}' ~/.ssh/config 2>/dev/null) 2>/dev/null";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssh_hosts_parses_host_entries_and_skips_wildcards() {
        let config = "Host main-host\n  HostName 10.0.0.1\nHost *.internal\nHost included-host\n  HostName 10.0.0.9\n";
        let hosts: Vec<String> = ssh_hosts(config)
            .suggestions
            .into_iter()
            .map(|s| s.exact_string)
            .collect();
        assert_eq!(hosts, vec!["main-host", "included-host"]);
    }

    // The Include-path expansion lives in an awk program embedded in `SSH_CONFIG_CMD`.
    // These tests run that exact program (extracted between the single quotes, so there is
    // a single source of truth) over a synthetic config and assert how each Include style is
    // resolved. They are gated to unix because they shell out to `awk`.
    #[cfg(unix)]
    mod include_expansion {
        use super::*;
        use std::io::Write;
        use std::process::Command;
        use std::sync::atomic::{AtomicU64, Ordering};

        /// The awk program shipped inside `SSH_CONFIG_CMD` (the text between its single quotes).
        /// This relies on the command embedding exactly one single-quoted awk program and the
        /// program itself containing no single quote -- both true today (awk uses double quotes
        /// internally). It keeps the test in lockstep with whatever the generator actually ships.
        fn include_awk() -> &'static str {
            SSH_CONFIG_CMD
                .split('\'')
                .nth(1)
                .expect("SSH_CONFIG_CMD embeds an awk program in single quotes")
        }

        /// Runs the shipped awk over `config_body` with `$HOME=home` and returns the
        /// Include paths it resolves (one per emitted line).
        fn resolved_includes(config_body: &str, home: &str) -> Vec<String> {
            // Cargo runs tests as threads in one process, so a pid-only filename would be
            // shared across the concurrent cases; a per-call counter keeps each unique.
            static SEQ: AtomicU64 = AtomicU64::new(0);
            let mut tmp = std::env::temp_dir();
            tmp.push(format!(
                "cs_ssh_include_{}_{}.cfg",
                std::process::id(),
                SEQ.fetch_add(1, Ordering::Relaxed)
            ));
            {
                let mut f = std::fs::File::create(&tmp).expect("create temp config");
                f.write_all(config_body.as_bytes())
                    .expect("write temp config");
            }
            let out = Command::new("awk")
                .arg(include_awk())
                .arg(&tmp)
                .env("HOME", home)
                .output()
                .expect("run awk");
            let _ = std::fs::remove_file(&tmp);
            assert!(
                out.status.success(),
                "awk failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| l.to_string())
                .collect()
        }

        #[test]
        fn normalizes_windows_backslash_drive_path() {
            assert_eq!(
                resolved_includes("Include C:\\Users\\me\\.ssh\\extra\n", "/home/me"),
                vec!["/c/Users/me/.ssh/extra"]
            );
        }

        #[test]
        fn normalizes_windows_forward_slash_drive_path() {
            assert_eq!(
                resolved_includes("Include D:/data/ssh_extra\n", "/home/me"),
                vec!["/d/data/ssh_extra"]
            );
        }

        #[test]
        fn leaves_posix_absolute_path_unchanged() {
            assert_eq!(
                resolved_includes("Include /etc/ssh/extra\n", "/home/me"),
                vec!["/etc/ssh/extra"]
            );
        }

        #[test]
        fn resolves_relative_path_under_ssh_dir() {
            assert_eq!(
                resolved_includes("Include work_config\n", "/home/me"),
                vec!["/home/me/.ssh/work_config"]
            );
        }

        #[test]
        fn expands_tilde_to_home() {
            assert_eq!(
                resolved_includes("Include ~/.ssh/extra\n", "/home/me"),
                vec!["/home/me/.ssh/extra"]
            );
        }
    }
}
