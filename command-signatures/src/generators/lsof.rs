use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("lsof")
        .add_generator(
            "process_names",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command("ps -A -o comm"),
                    CommandBuilder::single_command("sort -u"),
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|path| {
                            let name = path.rsplit('/').next().unwrap_or(path);
                            if !name.is_empty() {
                                Some(Suggestion::new(name))
                            } else {
                                None
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "pids",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command("ps axo pid,comm"),
                    CommandBuilder::single_command("sed 1d"),
                ),
                |output| {
                    output
                        .lines()
                        .filter_map(|line| {
                            let mut result = line.split_whitespace();

                            result
                                .next()
                                .zip(result.next())
                                .map(|(pid, path)| Suggestion::with_description(pid, path))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "users",
            Generator::script(
                // Cross-platform: getent (Linux) -> dscl (macOS) -> /etc/passwd (fallback)
                CommandBuilder::single_command(
                    "sh -c 'if command -v getent >/dev/null 2>&1; then getent passwd | cut -d: -f1; elif command -v dscl >/dev/null 2>&1; then dscl . -list /Users; else cut -d: -f1 /etc/passwd; fi'",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty() && !line.starts_with('_') && !line.starts_with('#'))
                        .map(|name| Suggestion::with_description(name.trim(), "User"))
                        .collect_unordered_results()
                },
            ),
        )
}
