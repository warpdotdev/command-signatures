use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("op").add_generator(
        "account_list_json",
        Generator::script(
            CommandBuilder::single_command("op account list --format json"),
            fig_parse::lines,
        ),
    )
}
