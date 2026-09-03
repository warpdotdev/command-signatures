use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("watson")
        .add_generator(
            "log",
            Generator::script(
                CommandBuilder::single_command("watson log --json --reverse"),
                output_parsers::json_string_array,
            ),
        )
        .add_generator(
            "projects",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                output_parsers::named_lines,
            ),
        )
        .add_generator(
            "tags",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                output_parsers::named_lines,
            ),
        )
}
