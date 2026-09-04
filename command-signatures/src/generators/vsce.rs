use super::template_filters;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vsce")
        .add_generator(
            "git_branch",
            Generator::script(
                CommandBuilder::single_command(
                    "git --no-optional-locks branch -a --no-color --sort=-committerdate",
                ),
                crate::generators::git::post_process_branches,
            ),
        )
        .add_filter("filter-vsix", template_filters::vsix())
}
