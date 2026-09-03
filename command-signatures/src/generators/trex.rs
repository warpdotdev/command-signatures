use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("trex")
        .add_generator(
            "cat_import_map_json",
            Generator::script(
                CommandBuilder::single_command("cat import_map.json"),
                output_parsers::trex_imports,
            ),
        )
        .add_generator(
            "cat_run_json",
            Generator::script(
                CommandBuilder::single_command("cat run.json"),
                output_parsers::trex_scripts,
            ),
        )
}
