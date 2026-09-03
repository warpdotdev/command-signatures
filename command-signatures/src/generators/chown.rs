use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("chown").add_generator(
        "users_or_groups",
        Generator::command_from_tokens(fig_token::chown_dscl, output_parsers::lines),
    )
}
