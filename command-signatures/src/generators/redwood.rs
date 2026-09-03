use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("redwood").add_generator(
        "until_redwood_toml_do_cd",
        Generator::script(
            CommandBuilder::single_command(
                "until [[ -f redwood.toml ]] || [[ $PWD = '/' ]]; do cd ..; done; ls -1p scripts/",
            ),
            fig_parse::lines,
        ),
    )
}
