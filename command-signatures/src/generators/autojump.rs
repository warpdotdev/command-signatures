use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("j").add_generator(
        "cat",
        Generator::script(
            CommandBuilder::single_command(
                r#"cat "$HOME/Library/autojump/autojump.txt" "$HOME/.local/share/autojump/autojump.txt" 2>/dev/null"#,
            ),
            output_parsers::named_lines,
        ),
    )
}
