use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pandoc")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("pandoc --list-output-formats"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions_list",
            Generator::script(
                CommandBuilder::single_command("pandoc --list-input-formats"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "pandoc",
            Generator::script(
                CommandBuilder::single_command(
                    "pandoc --list-input-formats && pandoc --list-output-formats",
                ),
                fig_parse::lines,
            ),
        )
}
