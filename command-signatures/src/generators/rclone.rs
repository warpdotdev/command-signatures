use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rclone").add_generator(
        "listremotes",
        Generator::script(
            CommandBuilder::single_command("rclone listremotes"),
            fig_parse::lines,
        ),
    )
}
