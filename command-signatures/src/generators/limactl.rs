use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("limactl").add_generator(
        "list",
        Generator::script(
            CommandBuilder::single_command("limactl list --quiet"),
            fig_parse::lines,
        ),
    )
}
