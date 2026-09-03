use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ignite-cli").add_generator(
        "ls_ignite_templates",
        Generator::script(
            CommandBuilder::single_command("ls ignite/templates"),
            fig_parse::lines,
        ),
    )
}
