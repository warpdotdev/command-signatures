use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fig-teams@latest").add_generator(
        "npx_fig_teams_latest_teams",
        Generator::script(
            CommandBuilder::single_command("npx -y fig-teams@latest teams ls --json"),
            fig_parse::lines,
        ),
    )
}
