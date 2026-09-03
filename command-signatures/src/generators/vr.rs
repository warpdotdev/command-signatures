use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vr")
        .add_generator(
            "completions_color",
            Generator::script(
                CommandBuilder::single_command("NO_COLOR=1 vr"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions_no",
            Generator::script(
                CommandBuilder::single_command("NO_COLOR=1 vr"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("NO_COLOR=1 vr"),
                fig_parse::lines,
            ),
        )
}
