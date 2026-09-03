use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mackup").add_generator(
        "list",
        Generator::script(
            CommandBuilder::single_command("mackup list"),
            output_parsers::named_lines,
        ),
    )
}
