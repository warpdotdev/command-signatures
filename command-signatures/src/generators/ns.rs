use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ns")
        .add_generator(
            "nativescript_templates",
            Generator::script(
                CommandBuilder::single_command("curl https://api.github.com/repos/NativeScript/nativescript-app-templates/contents/packages"),
                output_parsers::json_nativescript_templates,
            ),
        )
}
