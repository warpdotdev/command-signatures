use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("npx")
        .add_generator(
            "until_node_modules_do_cd",
            Generator::script(
                CommandBuilder::single_command("until [[ -d node_modules/ ]] || [[ $PWD = '/' ]]; do cd ..; done; ls -1 node_modules/.bin/"),
                fig_parse::lines,
            ),
        )
}
