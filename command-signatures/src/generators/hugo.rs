use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("hugo").add_generator(
        "ls_archetypes",
        Generator::script(
            CommandBuilder::single_command("ls ./archetypes/"),
            fig_parse::lines,
        ),
    )
}
