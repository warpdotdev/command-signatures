use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("killall")
        .add_generator(
            "user_name",
            Generator::script(
                CommandBuilder::single_command("dscl . -list /Users 2>/dev/null | grep -v '^_'"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .map(Suggestion::new)
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "process_name",
            Generator::script(
                CommandBuilder::single_command("ps -A -o comm 2>/dev/null | sort -u"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|path| {
                            path.rsplit_once('/').and_then(|(_, name)| {
                                if !name.is_empty() {
                                    Some(Suggestion::with_description(name, path))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
