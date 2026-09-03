use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tailscale")
        .add_generator(
            "status",
            Generator::script(
                CommandBuilder::single_command("tailscale status --json"),
                output_parsers::tailscale_peers,
            ),
        )
        .add_generator(
            "peers_colon",
            Generator::script(
                CommandBuilder::single_command("tailscale status --json"),
                output_parsers::tailscale_peers_colon,
            ),
        )
}
