use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ip")
        .add_generator(
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
        .add_generator(
            "interfaces",
            Generator::script(
                CommandBuilder::single_command("ip -o link show 2>/dev/null"),
                |output| {
                    output
                        .lines()
                        .filter_map(|line| {
                            // Output format: "N: name[@peer]: <FLAGS> ..."
                            let after_index = line.split_once(':')?.1;
                            let name = after_index.split_once(':')?.0.trim();
                            // Strip the "@peer" suffix that veth-style devices report.
                            let name = name.split('@').next().unwrap_or(name);
                            if name.is_empty() {
                                return None;
                            }
                            Some(Suggestion::with_description(name, "Network interface"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
