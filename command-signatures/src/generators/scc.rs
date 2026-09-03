use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("scc")
        .add_generator(
            "completions_2",
            Generator::script(
                CommandBuilder::single_command("scc --languages"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions_languages",
            Generator::script(
                CommandBuilder::single_command("scc --languages"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions_scc",
            Generator::script(
                CommandBuilder::single_command("scc --languages"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("scc --languages"),
                fig_parse::lines,
            ),
        )
}
