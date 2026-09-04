use super::common;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ssh")
        .add_generator("hosts", common::ssh_hosts_generator())
        .add_generator("addresses", common::ssh_hosts_generator())
        .add_generator(
            "known_hosts",
            Generator::script(
                CommandBuilder::single_command("cat ~/.ssh/known_hosts"),
                super::output_parsers::ssh_known_hosts,
            ),
        )
}
