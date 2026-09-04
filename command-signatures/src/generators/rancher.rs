use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rancher")
        .add_generator(
            "server",
            Generator::script(
                CommandBuilder::single_command("rancher server ls"),
                output_parsers::named_lines,
            ),
        )
        .add_filter("filter-json-yaml", template_filters::json_yaml())
}
