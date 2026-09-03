use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rush").add_generator(
        "until_rush_json_do_cd",
        Generator::script(
            CommandBuilder::single_command(
                "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
            ),
            fig_parse::lines,
        ),
    )
}
