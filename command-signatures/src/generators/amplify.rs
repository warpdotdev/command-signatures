use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("amplify")
        .add_generator(
            "env_list_2",
            Generator::script(
                CommandBuilder::single_command("amplify env list --json"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "env_list_json",
            Generator::script(
                CommandBuilder::single_command("amplify env list --json"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "env_list_amplify",
            Generator::script(
                CommandBuilder::single_command("amplify env list --json"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "env_list",
            Generator::script(
                CommandBuilder::single_command("amplify env list --json"),
                fig_parse::lines,
            ),
        )
}
