use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("dtm").add_generator(
        "list_plugins",
        Generator::script(
            CommandBuilder::single_command("dtm list plugins"),
            fig_parse::lines,
        ),
    )
}
