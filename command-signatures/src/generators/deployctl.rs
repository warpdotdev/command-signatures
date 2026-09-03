use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deployctl").add_generator(
        "curl_https_cdn_deno_land_deploy_meta_versions_json",
        Generator::script(
            CommandBuilder::single_command(
                "curl -sL 'https://cdn.deno.land/deploy/meta/versions.json'",
            ),
            fig_parse::lines,
        ),
    )
}
