use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("amplify").add_generator(
        "env_list",
        Generator::script(
            CommandBuilder::single_command("amplify env list --json"),
            output_parsers::json_envs,
        ),
    )
}
