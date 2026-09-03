use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rushx").add_generator(
        "until_package_json_do_cd",
        Generator::script(
            CommandBuilder::single_command(
                "until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json",
            ),
            output_parsers::json_script_keys,
        ),
    )
}
