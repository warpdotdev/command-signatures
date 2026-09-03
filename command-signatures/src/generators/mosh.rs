use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mosh")
        .add_generator(
            "cat_ssh_config",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/config"),
                output_parsers::ssh_hosts,
            ),
        )
        .add_generator(
            "cat_ssh_known_hosts",
            Generator::command_from_tokens(
                super::fig_token::known_hosts_file,
                output_parsers::ssh_known_hosts,
            ),
        )
}
