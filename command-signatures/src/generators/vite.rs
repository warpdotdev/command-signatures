use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vite").add_generator(
        "ls",
        Generator::script(
            CommandBuilder::single_command(r"\ls -l1A.env.*"),
            fig_parse::lines,
        ),
    )
}
