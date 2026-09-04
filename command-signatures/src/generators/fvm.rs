use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fvm").add_generator(
        "releases",
        Generator::script(
            CommandBuilder::single_command("fvm releases"),
            output_parsers::unique_named_lines,
        ),
    )
}
