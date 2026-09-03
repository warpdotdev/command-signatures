use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("yo").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("yo --generators"),
            fig_parse::lines,
        ),
    )
}
