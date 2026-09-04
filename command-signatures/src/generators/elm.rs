use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("elm")
        .add_generator(
            "elm_packages",
            Generator::script(
                CommandBuilder::single_command("curl -sH 'accept-encoding: gzip' https://package.elm-lang.org/search.json | gunzip"),
                output_parsers::json_name_summary,
            ),
        )
}
