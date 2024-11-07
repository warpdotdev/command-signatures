use warp_completion_metadata::{CommandSignatureGenerators, Generator};

use super::git::{post_process_branches, post_process_commits};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gt")
        .add_generator(
            "commits",
            Generator::script(
                "git --no-optional-locks log --oneline",
                post_process_commits,
            ),
        )
        .add_generator(
            "local_branches",
            Generator::script(
                "git --no-optional-locks branch --no-color --sort=-committerdate",
                post_process_branches,
            ),
        )
}
