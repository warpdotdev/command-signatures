use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("docker-compose").add_generator(
        "compose_services",
        Generator::command_from_tokens(
            |tokens, _has_trailing_whitespace, _| {
                // Extract -f/--file arguments from the token list so the generator
                // queries the correct compose file(s) when the user overrides the
                // default location.
                let mut file_args: Vec<&str> = Vec::new();
                let mut i = 0;
                while i < tokens.len().saturating_sub(1) {
                    if tokens[i] == "-f" || tokens[i] == "--file" {
                        file_args.push(tokens[i + 1]);
                        i += 2;
                    } else {
                        i += 1;
                    }
                }

                if file_args.is_empty() {
                    CommandBuilder::single_command("docker-compose config --services")
                } else {
                    let file_flags = file_args
                        .iter()
                        .map(|f| format!("-f '{}'", f))
                        .collect::<Vec<_>>()
                        .join(" ");
                    CommandBuilder::single_command(format!(
                        "docker-compose {} config --services",
                        file_flags
                    ))
                }
            },
            |output| {
                output
                    .trim()
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| Suggestion::new(line.trim()))
                    .collect_unordered_results()
            },
        ),
    )
}
