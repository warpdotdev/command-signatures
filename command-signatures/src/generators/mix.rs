use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mix").add_generator(
        "help",
        Generator::script(
            CommandBuilder::single_command("mix help"),
            output_parsers::lines,
        ),
    )
}
