use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pre-commit")
        .add_generator(
            "cat_pre_commit_config_yaml",
            Generator::script(
                CommandBuilder::single_command("cat .pre-commit-config.yaml"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch --no-color --sort=-committerdate",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_branch_no",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch -a --no-color --sort=-committerdate",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_remote",
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks remote -v"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "git_rev_list",
            Generator::script(
                CommandBuilder::single_command("git rev-list --all --oneline"),
                fig_parse::lines,
            ),
        )
}
