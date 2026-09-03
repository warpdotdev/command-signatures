use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("robot").add_generator(
        "for_i_in_robot",
        Generator::script(
            CommandBuilder::single_command(
                r#"for i in $(find -E . -regex ".*.robot" -type f); do cat -s $i ; done"#,
            ),
            fig_parse::lines,
        ),
    )
}
