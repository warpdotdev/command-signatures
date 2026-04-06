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
