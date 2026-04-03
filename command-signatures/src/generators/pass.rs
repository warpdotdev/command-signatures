use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pass")
        .add_generator(
            "entries",
            Generator::script(
                CommandBuilder::single_command(
                    "sh -c 'p=\"${PASSWORD_STORE_DIR:-$HOME/.password-store}\"; echo \"$p\"; find \"$p\" -name .git -prune -o -name \"*.gpg\" -print 2>/dev/null'",
                ),
                parse_entries,
            ),
        )
        .add_generator(
            "entry_dirs",
            Generator::script(
                CommandBuilder::single_command(
                    "sh -c 'p=\"${PASSWORD_STORE_DIR:-$HOME/.password-store}\"; echo \"$p\"; find \"$p\" -mindepth 1 -name .git -prune -o -type d -print 2>/dev/null'",
                ),
                parse_entry_dirs,
            ),
        )
}

/// Parses the output of the entries generator command.
/// The first line is the password store prefix; subsequent lines are full paths to `.gpg` files.
fn parse_entries(output: &str) -> GeneratorResults {
    let mut lines = output.trim().lines();
    let prefix = match lines.next() {
        Some(p) => p,
        None => return GeneratorResults::default(),
    };
    let prefix_with_slash = format!("{prefix}/");
    lines
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.strip_prefix(&prefix_with_slash)
                .and_then(|entry| entry.strip_suffix(".gpg"))
                .map(|entry| Suggestion::with_description(entry, "Password entry"))
        })
        .collect_unordered_results()
}

/// Parses the output of the entry_dirs generator command.
/// The first line is the password store prefix; subsequent lines are full paths to directories.
fn parse_entry_dirs(output: &str) -> GeneratorResults {
    let mut lines = output.trim().lines();
    let prefix = match lines.next() {
        Some(p) => p,
        None => return GeneratorResults::default(),
    };
    let prefix_with_slash = format!("{prefix}/");
    lines
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.strip_prefix(&prefix_with_slash)
                .map(|dir| Suggestion::with_description(dir, "Directory"))
        })
        .collect_unordered_results()
}
