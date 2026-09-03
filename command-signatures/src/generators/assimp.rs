use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("assimp")
        .add_generator(
            "listexport",
            Generator::script(
                CommandBuilder::single_command("assimp listexport"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listext",
            Generator::script(
                CommandBuilder::single_command("assimp listext"),
                fig_parse::lines,
            ),
        )
}
