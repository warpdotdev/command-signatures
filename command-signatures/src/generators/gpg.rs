use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gpg").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("gpg --version"),
            fig_parse::lines,
        ),
    )
}
