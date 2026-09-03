use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("hyper").add_generator(
        "list",
        Generator::script(
            CommandBuilder::single_command("hyper list"),
            fig_parse::lines,
        ),
    )
}
