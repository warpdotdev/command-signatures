use super::common;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ssh")
        .add_generator("hosts", common::ssh_hosts_generator())
        .add_generator("addresses", common::ssh_hosts_generator())
        .add_generator(
            "known_hosts",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/known_hosts"),
                |output| {
                    output
                        .lines()
                        .filter_map(|line| line.split_once(' ').map(|(first, _)| first))
                        .map(|known_host| Suggestion::with_description(known_host, "SSH Host"))
                        .collect_unordered_results()
                },
            ),
        )
}
