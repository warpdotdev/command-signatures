use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("terragrunt")
        .add_generator(
            "state_list",
            Generator::script(
                CommandBuilder::single_command("terragrunt state list"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "workspace_list",
            Generator::script(
                CommandBuilder::single_command("terragrunt workspace list"),
                fig_parse::lines,
            ),
        )
}
