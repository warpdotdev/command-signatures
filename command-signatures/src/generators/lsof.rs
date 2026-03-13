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
                CommandBuilder::single_command("cat /etc/passwd"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.starts_with('#'))
                        .filter_map(|line| {
                            line.split(':')
                                .next()
                                .map(|name| Suggestion::with_description(name, "User"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
