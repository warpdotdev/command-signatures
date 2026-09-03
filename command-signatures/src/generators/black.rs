use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("black").add_generator(
        "gh_release_list_psf_black",
        Generator::script(
            CommandBuilder::single_command("gh release list --repo psf/black"),
            fig_parse::lines,
        ),
    )
}
