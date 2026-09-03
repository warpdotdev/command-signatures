use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("okteto")
        .add_generator(
            "context_list",
            Generator::script(
                CommandBuilder::single_command("okteto context list"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "namespace_list",
            Generator::script(
                CommandBuilder::single_command("okteto namespace list"),
                fig_parse::lines,
            ),
        )
}
