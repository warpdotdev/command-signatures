use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("wsl")
        .add_generator(
            "distros",
            Generator::script(
                CommandBuilder::single_command("wsl.exe --list --quiet"),
                |output| {
                    output
                        .lines()
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::new(line))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "online_distros",
            Generator::script(
                CommandBuilder::single_command("wsl.exe --list --online"),
                |output| {
                    output
                        .lines()
                        .skip(4) // Skip header lines
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .filter_map(|line| {
                            let mut parts = line.splitn(2, char::is_whitespace);
                            let name = parts.next()?;
                            let description = parts
                                .next()
                                .map(|s| s.trim())
                                .unwrap_or("");
                            if description.is_empty() {
                                Some(Suggestion::new(name))
                            } else {
                                Some(Suggestion::with_description(name, description))
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
