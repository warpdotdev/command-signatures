use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("n")
        .add_generator(
            "lsr_5",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr_4",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr_3",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr_2",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr_all",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr_n",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lsr",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                fig_parse::lines,
            ),
        )
}
