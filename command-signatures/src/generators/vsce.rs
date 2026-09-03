use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vsce").add_generator(
        "git_branch",
        Generator::script(
            CommandBuilder::single_command(
                "git --no-optional-locks branch -a --no-color --sort=-committerdate",
            ),
            fig_parse::lines,
        ),
    )
}
