use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("wd")
        .add_generator(
            "cat_warprc_5",
            Generator::script(
                CommandBuilder::single_command("cat ~/.warprc"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_warprc_4",
            Generator::script(
                CommandBuilder::single_command("cat ~/.warprc"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_warprc_3",
            Generator::script(
                CommandBuilder::single_command("cat ~/.warprc"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_warprc_2",
            Generator::script(
                CommandBuilder::single_command("cat ~/.warprc"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_warprc",
            Generator::script(
                CommandBuilder::single_command("cat ~/.warprc"),
                fig_parse::lines,
            ),
        )
}
