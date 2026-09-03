use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tailscale").add_generator(
        "status",
        Generator::script(
            CommandBuilder::single_command("tailscale status --json"),
            output_parsers::lines,
        ),
    )
}
