use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("open")
        .add_generator(
            "mdfind",
            Generator::script(
                CommandBuilder::single_command("mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin /"),
                output_parsers::named_lines,
            ),
        )
        .add_generator(
            "mdfind_while_read_line",
            Generator::script(
                CommandBuilder::single_command(r#"mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin / | while read line; do echo $(mdls -name kMDItemCFBundleIdentifier -r "$line") $line; done"#),
                output_parsers::named_lines,
            ),
        )
}
