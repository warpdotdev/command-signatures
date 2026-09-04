use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vultr-cli").add_generator(
        "instance_list",
        Generator::script(
            CommandBuilder::single_command("vultr-cli instance list"),
            output_parsers::named_lines,
        ),
    )
}
