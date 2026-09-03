use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kool")
        .add_generator(
            "docker_compose_config",
            Generator::script(
                CommandBuilder::single_command("docker-compose config --services"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "run",
            Generator::script(
                CommandBuilder::single_command("kool run --help"),
                fig_parse::lines,
            ),
        )
}
