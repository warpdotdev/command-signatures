use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("flyctl")
        .add_generator(
            "list_apps",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                output_parsers::pipe_table_name_col1_desc,
            ),
        )
        .add_generator(
            "list_orgs",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                output_parsers::pipe_table_name_col1_desc,
            ),
        )
}
