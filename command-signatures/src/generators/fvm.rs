use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fvm").add_generator(
        "releases",
        Generator::script(
            CommandBuilder::single_command("fvm releases"),
            fig_parse::lines,
        ),
    )
}
