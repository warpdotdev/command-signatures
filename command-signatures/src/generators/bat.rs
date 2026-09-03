use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("bat")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("bat --list-themes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "completions_bat",
            Generator::script(
                CommandBuilder::single_command("bat --list-languages"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "unknow_dev_null_possible",
            Generator::script(
                CommandBuilder::single_command(
                    "bat --paging unknow  2>&1 >/dev/null | grep possible",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "unknow_dev_null_possible_bat",
            Generator::script(
                CommandBuilder::single_command(
                    "bat --decorations unknow  2>&1 >/dev/null | grep possible",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "unknow_dev_null_possible_color",
            Generator::script(
                CommandBuilder::single_command(
                    "bat --color unknow  2>&1 >/dev/null | grep possible",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "unknow_dev_null_possible_italic",
            Generator::script(
                CommandBuilder::single_command(
                    "bat --italic-text unknow  2>&1 >/dev/null | grep possible",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "unknow_dev_null_possible_wrap",
            Generator::script(
                CommandBuilder::single_command(
                    "bat --wrap unknow  2>&1 >/dev/null | grep possible",
                ),
                fig_parse::lines,
            ),
        )
}
