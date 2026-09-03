use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rancher").add_generator(
        "server",
        Generator::script(
            CommandBuilder::single_command("rancher server ls"),
            output_parsers::lines,
        ),
    )
}
