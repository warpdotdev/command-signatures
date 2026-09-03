use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("watson")
        .add_generator(
            "log",
            Generator::script(
                CommandBuilder::single_command("watson log --json --reverse"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
}
