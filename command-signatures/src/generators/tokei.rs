use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tokei").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("tokei --languages"),
            fig_parse::lines,
        ),
    )
}
