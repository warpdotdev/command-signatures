use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tccutil")
        .add_generator(
            "mdfind_applications_while_read",
            Generator::script(
                CommandBuilder::single_command(r#"mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin /Applications | while read line; do echo $(mdls -name kMDItemCFBundleIdentifier -r "$line") $line; done"#),
                output_parsers::named_lines,
            ),
        )
}
