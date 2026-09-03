use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("meteor")
        .add_generator(
            "cat_meteor_packages",
            Generator::script(
                CommandBuilder::single_command("cat ./.meteor/packages"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "create",
            Generator::script(
                CommandBuilder::single_command("meteor create --list"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_platforms",
            Generator::script(
                CommandBuilder::single_command("meteor list-platforms"),
                fig_parse::lines,
            ),
        )
}
