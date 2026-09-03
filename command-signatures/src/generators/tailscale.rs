use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tailscale").add_generator(
        "status",
        Generator::script(
            CommandBuilder::single_command("tailscale status --json"),
            fig_parse::lines,
        ),
    )
}
