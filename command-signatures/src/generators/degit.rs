use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("degit").add_generator(
        "github_user_repos",
        Generator::command_from_tokens(
            fig_token::github_user_repos,
            output_parsers::github_repos_json,
        ),
    )
}
