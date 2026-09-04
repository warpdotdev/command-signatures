use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deta").add_generator(
        "node12_node14_python3_7_python3",
        Generator::script(
            CommandBuilder::single_command("echo node12, node14, python3.7, python3.9"),
            output_parsers::desc_runtime,
        ),
    )
}
