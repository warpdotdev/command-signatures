use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("assimp")
        .add_generator(
            "listexport",
            Generator::script(
                CommandBuilder::single_command("assimp listexport"),
                output_parsers::desc_extension,
            ),
        )
        .add_generator(
            "listext",
            Generator::script(
                CommandBuilder::single_command("assimp listext"),
                output_parsers::desc_extension,
            ),
        )
}
