use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("n").add_generator(
        "lsr",
        Generator::script(
            CommandBuilder::single_command("n lsr --all"),
            fig_parse::lines,
        ),
    )
}
