use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("eslint")
        .add_generator(
            "ls_node_modules_root_global",
            Generator::script(
                CommandBuilder::single_command("{ ls node_modules ; ls $(npm root -g) ; ls $(yarn global dir)/node_modules/ ; } | cat"),
                output_parsers::eslint_plugin_names,
            ),
        )
        .add_generator(
            "env_remaining",
            Generator::command_from_tokens(super::fig_token::eslint_env_remaining, output_parsers::named_lines),
        )
}
