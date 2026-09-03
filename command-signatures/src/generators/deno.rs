use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deno")
        .add_generator(
            "curl_https_cdn_deno_land_deno_meta_versions_json",
            Generator::script(
                CommandBuilder::single_command(
                    "curl -sL 'https://cdn.deno.land/deno/meta/versions.json'",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "find_deno_bin_1_f",
            Generator::script(
                CommandBuilder::single_command(r"\find ~/.deno/bin -maxdepth 1 -perm -111 -type f"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "lint",
            Generator::script(
                CommandBuilder::single_command("deno lint --rules --json"),
                fig_parse::lines,
            ),
        )
}
