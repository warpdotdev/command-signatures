use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("limactl")
        .add_generator(
            "list",
            Generator::script(
                CommandBuilder::single_command("limactl list --quiet"),
                output_parsers::desc_instance,
            ),
        )
        .add_filter("filter-yml-yaml", template_filters::yml_yaml())
}
