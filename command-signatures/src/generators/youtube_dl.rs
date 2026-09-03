use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("youtube-dl").add_generator(
        "pbpaste",
        Generator::script(CommandBuilder::single_command("pbpaste"), fig_parse::lines),
    )
}
