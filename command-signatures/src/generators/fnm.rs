use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fnm")
        .add_generator(
            "ls",
            Generator::script(
                CommandBuilder::single_command("fnm ls"),
                fig_parse::slice2_reversed,
            ),
        )
        .add_generator(
            "ls_remote",
            Generator::script(
                CommandBuilder::single_command("fnm ls-remote"),
                fig_parse::second_whitespace_token,
            ),
        )
}
