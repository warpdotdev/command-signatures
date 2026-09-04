use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kool")
        .add_generator(
            "docker_compose_config",
            Generator::script(
                CommandBuilder::single_command("docker-compose config --services"),
                output_parsers::lines,
            ),
        )
        .add_generator(
            "run",
            Generator::script(
                CommandBuilder::single_command("kool run --help"),
                output_parsers::desc_script,
            ),
        )
}
