use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gpg").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("gpg --version"),
            output_parsers::gpg_ciphers,
        ),
    )
}
