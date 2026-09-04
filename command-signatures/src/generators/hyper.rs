use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("hyper").add_generator(
        "list",
        Generator::script(
            CommandBuilder::single_command("hyper list"),
            output_parsers::desc_plugin_name,
        ),
    )
}
