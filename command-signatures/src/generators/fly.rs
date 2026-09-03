use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fly")
        .add_generator(
            "flyctl_list_apps",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
}
