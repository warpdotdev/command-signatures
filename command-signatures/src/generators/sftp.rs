use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("sftp")
        .add_generator(
            "cat_ssh_config",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/config"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_ssh_known_hosts",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/known_hosts"),
                fig_parse::lines,
            ),
        )
}
