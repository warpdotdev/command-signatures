use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("eb")
        .add_generator(
            "list_5",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
        .add_generator(
            "list_4",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
        .add_generator(
            "list_3",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
        .add_generator(
            "list_2",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
        .add_generator(
            "list_eb",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
        .add_generator(
            "list",
            Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines),
        )
}
