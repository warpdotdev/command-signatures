use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("dtm")
        .add_generator(
            "list_plugins",
            Generator::script(
                CommandBuilder::single_command("dtm list plugins"),
                output_parsers::desc_plugin,
            ),
        )
        .add_filter("filter-yml-yaml", template_filters::yml_yaml())
}
