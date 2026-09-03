use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("okteto")
        .add_generator(
            "context_list",
            Generator::script(
                CommandBuilder::single_command("okteto context list"),
                output_parsers::lines_desc_context,
            ),
        )
        .add_generator(
            "namespace_list",
            Generator::script(
                CommandBuilder::single_command("okteto namespace list"),
                output_parsers::lines,
            ),
        )
}
