use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mix").add_generator(
        "help",
        Generator::script(CommandBuilder::single_command("mix help"), fig_parse::lines),
    )
}
