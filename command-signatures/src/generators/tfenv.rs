use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tfenv")
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("tfenv list"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_remote",
            Generator::script(
                CommandBuilder::single_command("tfenv list-remote"),
                fig_parse::lines,
            ),
        )
}
