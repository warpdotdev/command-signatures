use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("meteor")
        .add_generator(
            "cat_meteor_packages",
            Generator::script(
                CommandBuilder::single_command("cat ./.meteor/packages"),
                output_parsers::meteor_packages,
            ),
        )
        .add_generator(
            "create",
            Generator::script(
                CommandBuilder::single_command("meteor create --list"),
                output_parsers::meteor_examples,
            ),
        )
        .add_generator(
            "list_platforms",
            Generator::script(
                CommandBuilder::single_command("meteor list-platforms"),
                output_parsers::named_lines,
            ),
        )
        .add_filter("filter-json", template_filters::json())
}
