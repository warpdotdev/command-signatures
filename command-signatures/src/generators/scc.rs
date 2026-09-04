use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("scc")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("scc --languages"),
                output_parsers::scc_languages,
            ),
        )
        .add_generator(
            "format_multi",
            Generator::command_from_tokens(fig_token::scc_output_paths, output_parsers::lines),
        )
}
