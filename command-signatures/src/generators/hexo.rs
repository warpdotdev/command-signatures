use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("hexo").add_generator(
        "list_post_draft",
        Generator::script(
            CommandBuilder::single_command("hexo list post | grep -E ^Draft"),
            output_parsers::named_lines,
        ),
    )
}
