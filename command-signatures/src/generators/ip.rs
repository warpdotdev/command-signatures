use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ip").add_generator(
        "netns",
        Generator::script(
            CommandBuilder::single_command("ip netns list 2>/dev/null"),
            |output| {
                output
                    .lines()
                    .filter_map(|line| {
                        // Output format: "name" or "name (id: N)"
                        let name = line.split_whitespace().next()?;
                        if name.is_empty() {
                            return None;
                        }
                        Some(Suggestion::with_description(name, "Network namespace"))
                    })
                    .collect_unordered_results()
            },
        ),
    )
}
