use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("git-flow").add_generator(
        "type_branches",
        Generator::command_from_tokens(fig_token::git_flow_type_branches, output_parsers::lines),
    )
}
