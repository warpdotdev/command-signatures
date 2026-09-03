use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vultr-cli").add_generator(
        "instance_list",
        Generator::script(
            CommandBuilder::single_command("vultr-cli instance list"),
            fig_parse::lines,
        ),
    )
}
