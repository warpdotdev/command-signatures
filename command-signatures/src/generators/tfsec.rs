use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tfsec").add_generator(
        "terraform_workspace_list",
        Generator::script(
            CommandBuilder::single_command("terraform workspace list"),
            fig_parse::lines,
        ),
    )
}
