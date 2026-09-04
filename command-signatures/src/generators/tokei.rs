use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tokei").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("tokei --languages"),
            output_parsers::lines,
        ),
    )
}
