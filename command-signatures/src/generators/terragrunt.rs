use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("terragrunt")
        .add_generator(
            "state_list",
            Generator::script(
                CommandBuilder::single_command("terragrunt state list"),
                output_parsers::desc_address,
            ),
        )
        .add_generator(
            "workspace_list",
            Generator::script(
                CommandBuilder::single_command("terragrunt workspace list"),
                output_parsers::desc_workspace,
            ),
        )
}
