use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("projj")
        .add_generator(
            "cat_projj_cache_json",
            Generator::script(
                CommandBuilder::single_command("cat ~/.projj/cache.json"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "cat_projj_config_json",
            Generator::script(
                CommandBuilder::single_command("cat ~/.projj/config.json"),
                fig_parse::lines,
            ),
        )
}
