use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("meteor")
        .add_generator(
            "cat_meteor_packages",
            Generator::script(
                CommandBuilder::single_command("cat ./.meteor/packages"),
                output_parsers::lines,
            ),
        )
        .add_generator(
            "create",
            Generator::script(
                CommandBuilder::single_command("meteor create --list"),
                output_parsers::lines,
            ),
        )
        .add_generator(
            "list_platforms",
            Generator::script(
                CommandBuilder::single_command("meteor list-platforms"),
                output_parsers::lines,
            ),
        )
}
