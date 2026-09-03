use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("lerna")
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
            "list_while_read_p",
            Generator::script(
                CommandBuilder::single_command(
                    r"lerna list -p | while read p; do  \cat $p/package.json && echo END done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "ls",
            Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines),
        )
}
