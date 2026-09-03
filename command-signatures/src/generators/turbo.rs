use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("turbo")
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch -a --no-color --sort=-committerdate",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_turbo_json_do_cd",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ ( -f turbo.json || $PWD = '/' ) ]]; do cd ..; done; cat turbo.json",
                ),
                fig_parse::lines,
            ),
        )
}
