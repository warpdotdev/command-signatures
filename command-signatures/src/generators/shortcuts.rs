use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("shortcuts")
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("shortcuts list"),
                output_parsers::lines,
            ),
        )
        .add_generator(
            "list_shortcuts",
            Generator::script(
                CommandBuilder::single_command("shortcuts list --folders"),
                output_parsers::lines,
            ),
        )
}
