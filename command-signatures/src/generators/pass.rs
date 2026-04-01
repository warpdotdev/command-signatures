use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    // The shell commands cd into the password store directory so that `find`
    // outputs paths relative to it.  sed then strips the leading `./` and,
    // for entries, the trailing `.gpg` extension.
    CommandSignatureGenerators::new("pass")
        .add_generator(
            "entries",
            Generator::script(
                CommandBuilder::single_command(
                    "(cd \"${PASSWORD_STORE_DIR:-$HOME/.password-store}\" 2>/dev/null && find . -name '*.gpg' ! -path '*/.git/*' | sed 's|^\\./||;s|\\.gpg$||' | sort)",
                ),
                |output| {
                    output
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Password entry"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "directories",
            Generator::script(
                CommandBuilder::single_command(
                    "(cd \"${PASSWORD_STORE_DIR:-$HOME/.password-store}\" 2>/dev/null && find . -type d ! -name '.git' ! -path '*/.git/*' -mindepth 1 | sed 's|^\\./||' | sort)",
                ),
                |output| {
                    output
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|line| Suggestion::with_description(line.trim(), "Password directory"))
                        .collect_unordered_results()
                },
            ),
        )
}
