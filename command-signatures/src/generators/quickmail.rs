use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("quickmail").add_generator(
        "template_listall",
        Generator::script(
            CommandBuilder::single_command("quickmail template listall"),
            fig_parse::lines,
        ),
    )
}
