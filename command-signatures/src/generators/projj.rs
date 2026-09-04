use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("projj")
        .add_generator(
            "cat_projj_cache_json",
            Generator::script(
                CommandBuilder::single_command("cat ~/.projj/cache.json"),
                output_parsers::projj_cache_repos,
            ),
        )
        .add_generator(
            "cat_projj_config_json",
            Generator::script(
                CommandBuilder::single_command("cat ~/.projj/config.json"),
                output_parsers::projj_hooks,
            ),
        )
}
