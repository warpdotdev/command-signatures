use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("elm")
        .add_generator(
            "curl_accept_encoding_gzip_https_package_elm_lang_org_search_json",
            Generator::script(
                CommandBuilder::single_command("curl -sH 'accept-encoding: gzip' https://package.elm-lang.org/search.json | gunzip"),
                fig_parse::lines,
            ),
        )
}
