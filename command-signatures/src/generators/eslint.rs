use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("eslint")
        .add_generator(
            "ls_node_modules_root_global",
            Generator::script(
                CommandBuilder::single_command("{ ls node_modules ; ls $(npm root -g) ; ls $(yarn global dir)/node_modules/ ; } | cat"),
                fig_parse::lines,
            ),
        )
}
