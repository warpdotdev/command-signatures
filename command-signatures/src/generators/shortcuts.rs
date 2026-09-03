use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("shortcuts")
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("shortcuts list"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_shortcuts",
            Generator::script(
                CommandBuilder::single_command("shortcuts list --folders"),
                fig_parse::lines,
            ),
        )
}
