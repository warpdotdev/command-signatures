use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("checkov").add_generator(
        "git_branch",
        Generator::script(
            CommandBuilder::single_command("git branch --no-color"),
            output_parsers::lines,
        ),
    )
}
