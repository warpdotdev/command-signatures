use warp_completion_metadata::CommandSignatureGenerators;

use super::git::{commits_generator, local_branches_generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gt")
        .add_generator("commits", commits_generator())
        .add_generator("local_branches", local_branches_generator())
}
