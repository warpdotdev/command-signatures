use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rancher")
        .add_generator(
            "server_rancher",
            Generator::script(
                CommandBuilder::single_command("rancher server ls"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "server",
            Generator::script(
                CommandBuilder::single_command("rancher server ls"),
                fig_parse::lines,
            ),
        )
}
