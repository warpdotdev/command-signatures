use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deno")
        .add_generator(
            "deno_versions",
            Generator::script(
                CommandBuilder::single_command(
                    "curl -sL 'https://cdn.deno.land/deno/meta/versions.json'",
                ),
                output_parsers::json_string_array,
            ),
        )
        .add_generator(
            "deno_binaries",
            Generator::script(
                CommandBuilder::single_command(r"\find ~/.deno/bin -maxdepth 1 -perm -111 -type f"),
                output_parsers::deno_binaries,
            ),
        )
        .add_generator(
            "lint",
            Generator::script(
                CommandBuilder::single_command("deno lint --rules --json"),
                output_parsers::json_deno_codes,
            ),
        )
        .add_generator(
            "doc_json",
            Generator::command_from_tokens(
                super::fig_token::deno_doc_json,
                output_parsers::json_deno_doc_nodes,
            ),
        )
}
