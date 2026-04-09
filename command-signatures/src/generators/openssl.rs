use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("openssl")
        .add_generator(
            "subcommands",
            Generator::script(
                CommandBuilder::single_command(
                    "openssl list -1 -commands -cipher-commands -digest-commands 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::new(name.trim()))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "digest_algorithms",
            Generator::script(
                CommandBuilder::single_command("openssl list -1 -digest-commands 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Digest algorithm"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "cipher_algorithms",
            Generator::script(
                CommandBuilder::single_command("openssl list -1 -cipher-commands 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Cipher algorithm"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "ec_curves",
            Generator::script(
                CommandBuilder::single_command("openssl ecparam -list_curves 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            let parts: Vec<&str> = line.splitn(2, ':').collect();
                            let name = parts.first()?.trim();
                            if name.is_empty() {
                                return None;
                            }
                            let description = parts.get(1).map(|d| d.trim()).unwrap_or("EC curve");
                            Some(Suggestion::with_description(name, description))
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
