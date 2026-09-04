use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ni").add_generator(
        "npms_search",
        Generator::command_from_tokens(fig_token::npms_search, output_parsers::npms_search_results),
    )
}
