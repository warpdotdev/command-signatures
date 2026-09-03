use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("copilot").add_generator(
        "cat_copilot_workspace",
        Generator::script(
            CommandBuilder::single_command("cat copilot/.workspace"),
            fig_parse::lines,
        ),
    )
}
