use super::common;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

/// Extracts the `-P <port>` value from scp tokens if present.
fn find_port<'a>(tokens: &'a [&'a str]) -> Option<&'a str> {
    tokens
        .windows(2)
        .find(|pair| pair[0] == "-P")
        .map(|pair| pair[1])
}

/// Builds an SSH command to list remote files for the current token's host:path.
fn remote_paths_command(
    tokens: &[&str],
    trailing_whitespace: bool,
    _env: &[String],
) -> CommandBuilder {
    let current_token = if trailing_whitespace {
        ""
    } else {
        tokens.last().copied().unwrap_or("")
    };

    let Some((host, path_prefix)) = current_token.split_once(':') else {
        return CommandBuilder::single_command("true");
    };

    if host.is_empty() {
        return CommandBuilder::single_command("true");
    }

    let port_flag = find_port(tokens)
        .map(|p| format!(" -p {}", p))
        .unwrap_or_default();

    // Escape single quotes in the path prefix for safe embedding in a single-quoted shell string.
    let escaped_prefix = path_prefix.replace('\'', "'\\''");

    // Use `command ls -1dp` to list entries with trailing / on directories.
    // The glob `*` after the prefix is expanded by the remote shell (single quotes only
    // prevent expansion in the local shell; SSH passes the string to the remote shell).
    // When the prefix is empty (just `host:`), list the home directory contents.
    let ls_pattern = if escaped_prefix.is_empty() {
        String::from("*")
    } else {
        format!("{}*", escaped_prefix)
    };

    let ssh_cmd = format!(
        "ssh{} -o BatchMode=yes -o ConnectTimeout=5 {} 'command ls -1dp {} 2>/dev/null'",
        port_flag, host, ls_pattern
    );

    // Prefix each result line with `host:` so the completion replaces the full token.
    let sed_cmd = format!("sed 's|^|{}:|'", host);

    CommandBuilder::pipe(
        CommandBuilder::single_command(ssh_cmd),
        CommandBuilder::single_command(sed_cmd),
    )
}

fn remote_paths_results(output: &str) -> warp_completion_metadata::GeneratorResults {
    output
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.ends_with('/') {
                Suggestion::with_description(line, "Remote directory")
            } else {
                Suggestion::with_description(line, "Remote file")
            }
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("scp")
        .add_generator("hosts", common::ssh_hosts_generator())
        .add_generator(
            "remote_paths",
            Generator::command_from_tokens(remote_paths_command, remote_paths_results),
        )
}
