use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("mdfind").add_generator(
        "ls_library_saved_searches_savedsearch",
        Generator::script(
            CommandBuilder::single_command(r"ls -1A ~/Library/Saved\ Searches/*.savedSearch"),
            output_parsers::named_lines,
        ),
    )
}
