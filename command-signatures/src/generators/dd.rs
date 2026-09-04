use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("dd").add_generator(
        "conv_remaining",
        Generator::command_from_tokens(fig_token::dd_conv_remaining, output_parsers::lines),
    )
}
