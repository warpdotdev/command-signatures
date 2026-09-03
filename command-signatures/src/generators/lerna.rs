use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("lerna")
        .add_generator(
            "ls_12",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_11",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_10",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_9",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_8",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_7",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "list_while_read_p_lerna",
            Generator::script(
                CommandBuilder::single_command(
                    "lerna list -p | while read p; do  \\cat $p/package.json && echo END done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command("git branch --no-color"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_remote",
            Generator::script(
                CommandBuilder::single_command("git remote"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "ls_6",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_5",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_4",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "list_while_read_p",
            Generator::script(
                CommandBuilder::single_command(
                    "lerna list -p | while read p; do  \\cat $p/package.json && echo END done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "ls_3",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_2",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls_lerna",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
        .add_generator(
            "ls",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
}
