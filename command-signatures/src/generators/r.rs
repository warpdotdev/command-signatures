use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("r").add_generator(
        "rscript_libpaths",
        Generator::script(
            CommandBuilder::single_command("Rscript -e '.libPaths()'"),
            fig_parse::lines,
        ),
    )
}
