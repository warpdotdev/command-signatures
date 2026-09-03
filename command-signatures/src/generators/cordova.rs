use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("cordova")
        .add_generator(
            "cat_package_json",
            Generator::script(
                CommandBuilder::single_command("cat package.json"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "plugin_list",
            Generator::script(
                CommandBuilder::single_command("cordova plugin list"),
                fig_parse::lines,
            ),
        )
}
