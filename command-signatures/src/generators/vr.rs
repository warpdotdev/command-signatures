use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vr").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("NO_COLOR=1 vr"),
            output_parsers::second_whitespace_token,
        ),
    )
}
