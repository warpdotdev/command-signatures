use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fnm")
        .add_generator(
            "ls",
            Generator::script(
                CommandBuilder::single_command("fnm ls"),
                output_parsers::slice2_reversed,
            ),
        )
        .add_generator(
            "ls_remote",
            Generator::script(
                CommandBuilder::single_command("fnm ls-remote"),
                output_parsers::second_whitespace_token,
            ),
        )
}
