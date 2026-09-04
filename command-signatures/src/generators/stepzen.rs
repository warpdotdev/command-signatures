use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("stepzen")
        .add_generator(
            "github_schemas",
            Generator::script(
                CommandBuilder::single_command(
                    "curl https://api.github.com/repos/steprz/stepzen-schemas/contents",
                ),
                output_parsers::stepzen_github_dirs,
            ),
        )
        .add_generator(
            "list_schemas",
            Generator::script(
                CommandBuilder::single_command("stepzen list schemas"),
                output_parsers::stepzen_schema_names,
            ),
        )
}
