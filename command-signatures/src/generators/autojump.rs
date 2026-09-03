use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("autojump").add_generator(
        "cat",
        Generator::script(
            CommandBuilder::single_command("cat \"$HOME\"/Library/autojump/autojump.txt"),
            fig_parse::lines,
        ),
    )
}
