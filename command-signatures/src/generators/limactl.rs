use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("limactl")
        .add_generator(
            "list_5",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_4",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_3",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_2",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_quiet",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_limactl",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                fig_parse::lines,
            ),
        )
}
