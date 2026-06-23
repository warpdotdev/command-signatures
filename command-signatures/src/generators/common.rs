use serde::Deserialize;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandBuilder, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

/// Shell command that reads ~/.ssh/config and all readable files referenced by Include directives.
/// Include paths are resolved by replacing leading ~ with $HOME, treating relative paths as
/// relative to ~/.ssh/, and translating Windows drive-letter paths via cygpath when available.
/// Glob patterns in Include paths are expanded by the shell.
pub const SSH_CONFIG_CMD: &str = r#"sh -c 'emit(){ [ -r "$1" ] && cat "$1"; }; norm(){ p=$1; case "$p" in "~"|"~/"*) printf "%s\n" "$HOME${p#~}" ;; ?:/*|?:\\*) if command -v cygpath >/dev/null 2>&1; then cygpath -u "$p"; else drive=$(printf "%s" "$p" | cut -c1 | tr "[:upper:]" "[:lower:]"); rest=$(printf "%s" "$p" | cut -c3- | tr "\\\\" "/"); printf "/%s%s\n" "$drive" "$rest"; fi ;; /*) printf "%s\n" "$p" ;; *) printf "%s/.ssh/%s\n" "$HOME" "$p" ;; esac; }; config="$HOME/.ssh/config"; emit "$config"; while read -r keyword rest; do [ "$(printf "%s" "$keyword" | tr "[:upper:]" "[:lower:]")" = include ] || continue; set -- $rest; for include_path do norm "$include_path"; done; done < "$config" 2>/dev/null | while IFS= read -r resolved; do case "$resolved" in *[\*\?\[]*) for matched in $resolved; do emit "$matched"; done ;; *) emit "$resolved";; esac; done; true'"#;

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

#[cfg(all(test, unix))]
mod tests {
    use super::{ssh_hosts, SSH_CONFIG_CMD};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn unique_temp_dir() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "warp-command-signatures-ssh-config-{suffix}-{}-{counter}",
            std::process::id()
        ))
    }

    fn run_ssh_config_cmd(home: &Path, path_prefix: Option<&Path>) -> std::process::Output {
        let mut command = Command::new("sh");
        command.arg("-c").arg(SSH_CONFIG_CMD).env("HOME", home);
        if let Some(path_prefix) = path_prefix {
            let path = std::env::var("PATH").unwrap_or_default();
            command.env("PATH", format!("{}:{path}", path_prefix.display()));
        }
        command
            .output()
            .expect("ssh config generator command should run")
    }

    #[test]
    fn ssh_config_command_ignores_unreadable_includes() {
        let home = unique_temp_dir();
        fs::create_dir_all(home.join(".ssh")).expect("failed to create test ssh directory");
        fs::write(
            home.join(".ssh/config"),
            "Include missing_config\nHost base-host\n  HostName base.example\n",
        )
        .expect("failed to write test ssh config");

        let output = run_ssh_config_cmd(&home, None);

        fs::remove_dir_all(&home).expect("failed to remove test home");
        assert!(
            output.status.success(),
            "generator should tolerate missing Include paths: {:?}",
            output
        );
        let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
        assert!(stdout.contains("Host base-host"));
    }

    #[test]
    fn ssh_config_command_reads_windows_include_paths_with_cygpath() {
        let home = unique_temp_dir();
        let bin = home.join("bin");
        let ssh = home.join(".ssh");
        fs::create_dir_all(&bin).expect("failed to create test bin directory");
        fs::create_dir_all(&ssh).expect("failed to create test ssh directory");

        let extra_config = ssh.join("extra_config");
        fs::write(
            &extra_config,
            "Host included-host\n  HostName included.example\n",
        )
        .expect("failed to write included ssh config");
        fs::write(
            ssh.join("config"),
            "Include C:\\Users\\me\\.ssh\\extra_config\nHost base-host\n",
        )
        .expect("failed to write test ssh config");

        let cygpath = bin.join("cygpath");
        fs::write(
            &cygpath,
            format!(
                "#!/bin/sh\nfor arg do path=$arg; done\ncase \"$path\" in\n  'C:\\Users\\me\\.ssh\\extra_config') printf '%s\\n' '{}';;\n  *) exit 1;;\nesac\n",
                extra_config.display()
            ),
        )
        .expect("failed to write fake cygpath");
        fs::set_permissions(&cygpath, fs::Permissions::from_mode(0o755))
            .expect("failed to mark fake cygpath executable");

        let output = run_ssh_config_cmd(&home, Some(&bin));

        fs::remove_dir_all(&home).expect("failed to remove test home");
        assert!(
            output.status.success(),
            "generator should read cygpath-resolved Include paths: {:?}",
            output
        );
        let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
        let suggestions = ssh_hosts(&stdout).suggestions;
        assert!(
            suggestions
                .iter()
                .any(|suggestion| suggestion.exact_string == "base-host"),
            "base config host should still be present: {suggestions:?}"
        );
        assert!(
            suggestions
                .iter()
                .any(|suggestion| suggestion.exact_string == "included-host"),
            "Windows-style Include host should be present: {suggestions:?}"
        );
    }
}
