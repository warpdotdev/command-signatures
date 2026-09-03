use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("softwareupdate").add_generator(
        "completions",
        Generator::script(
            CommandBuilder::single_command("softwareupdate --list"),
            fig_parse::lines,
        ),
    )
}
