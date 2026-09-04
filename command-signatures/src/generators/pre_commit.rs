use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pre-commit")
        .add_generator(
            "cat_pre_commit_config_yaml",
            Generator::script(
                CommandBuilder::single_command("cat .pre-commit-config.yaml"),
                output_parsers::pre_commit_hook_ids,
            ),
        )
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch --no-color --sort=-committerdate",
                ),
                crate::generators::git::post_process_branches,
            ),
        )
        .add_generator(
            "git_branch_no",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch -a --no-color --sort=-committerdate",
                ),
                crate::generators::git::post_process_branches,
            ),
        )
        .add_generator(
            "git_remote",
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks remote -v"),
                output_parsers::desc_remote,
            ),
        )
        .add_generator(
            "git_rev_list",
            Generator::script(
                CommandBuilder::single_command("git rev-list --all --oneline"),
                output_parsers::git_oneline,
            ),
        )
}
