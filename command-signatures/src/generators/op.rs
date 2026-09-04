use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("op").add_generator(
        "account_list_json",
        Generator::script(
            CommandBuilder::single_command("op account list --format json"),
            output_parsers::op_accounts,
        ),
    )
}
