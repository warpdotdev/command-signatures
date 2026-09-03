use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("networkquality").add_generator(
        "networksetup",
        Generator::script(
            CommandBuilder::single_command("networksetup -listallhardwareports"),
            fig_parse::lines,
        ),
    )
}
