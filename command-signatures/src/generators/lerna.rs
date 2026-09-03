use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("lerna")
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command("git branch --no-color"),
                crate::generators::git::post_process_branches,
            ),
        )
        .add_generator(
            "git_remote",
            Generator::script(
                CommandBuilder::single_command("git remote"),
                output_parsers::desc_remote,
            ),
        )
        .add_generator(
            "list_while_read_p",
            Generator::script(
                CommandBuilder::single_command(
                    r"lerna list -p | while read p; do  \cat $p/package.json && echo END done",
                ),
                output_parsers::lerna_package_script_keys,
            ),
        )
        .add_generator(
            "ls",
            Generator::script(
                CommandBuilder::single_command("lerna ls"),
                output_parsers::named_lines,
            ),
        )
}
