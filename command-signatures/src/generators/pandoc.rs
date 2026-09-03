use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pandoc")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("pandoc --list-output-formats"),
                output_parsers::named_lines,
            ),
        )
        .add_generator(
            "completions_list",
            Generator::script(
                CommandBuilder::single_command("pandoc --list-input-formats"),
                output_parsers::named_lines,
            ),
        )
        .add_generator(
            "pandoc",
            Generator::script(
                CommandBuilder::single_command(
                    "pandoc --list-input-formats && pandoc --list-output-formats",
                ),
                output_parsers::unique_named_lines,
            ),
        )
        .add_filter("filter-yaml", template_filters::yaml())
        .add_filter("filter-yaml-json", template_filters::yaml_json())
}
