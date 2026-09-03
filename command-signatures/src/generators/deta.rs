use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deta")
        .add_generator(
            "echo_node12_node14_python3_7_9",
            Generator::script(
                CommandBuilder::single_command("echo node12, node14, python3.7, python3.9"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "echo_node12_node14_python3_7",
            Generator::script(
                CommandBuilder::single_command("echo node12, node14, python3.7, python3.9"),
                fig_parse::lines,
            ),
        )
}
