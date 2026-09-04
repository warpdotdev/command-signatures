use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mix")
        .add_generator(
            "help",
            Generator::script(
                CommandBuilder::single_command("mix help"),
                output_parsers::mix_help_tasks,
            ),
        )
        .add_filter("filter-exs", template_filters::exs())
}
