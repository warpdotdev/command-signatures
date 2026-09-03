use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("sftp")
        .add_generator(
            "cat_ssh_config",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/config"),
                output_parsers::ssh_hosts,
            ),
        )
        .add_generator(
            "cat_ssh_known_hosts",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/known_hosts"),
                output_parsers::lines,
            ),
        )
}
