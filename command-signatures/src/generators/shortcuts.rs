use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("shortcuts")
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("shortcuts list"),
                output_parsers::named_lines,
            ),
        )
        .add_generator(
            "list_shortcuts",
            Generator::script(
                CommandBuilder::single_command("shortcuts list --folders"),
                output_parsers::named_lines,
            ),
        )
        .add_filter("filter-shortcut", template_filters::shortcut())
}
