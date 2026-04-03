use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Shell command that reads ~/.ssh/config and all files referenced by Include directives.
/// Include paths are resolved by replacing ~ with $HOME and treating relative paths as
/// relative to ~/.ssh/. Glob patterns in Include paths are expanded by the shell.
const SSH_CONFIG_CMD: &str = "cat ~/.ssh/config $(awk 'tolower($1)==\"include\"{for(i=2;i<=NF;i++){gsub(\"~\",ENVIRON[\"HOME\"],$i);if($i!~/^\\//)$i=ENVIRON[\"HOME\"]\"/.ssh/\"$i;print $i}}' ~/.ssh/config 2>/dev/null) 2>/dev/null";

fn hosts(output: &str) -> GeneratorResults {
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

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ssh")
        .add_generator(
            "hosts",
            Generator::script(CommandBuilder::single_command(SSH_CONFIG_CMD), hosts),
        )
        .add_generator(
            "addresses",
            Generator::script(CommandBuilder::single_command(SSH_CONFIG_CMD), hosts),
        )
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
