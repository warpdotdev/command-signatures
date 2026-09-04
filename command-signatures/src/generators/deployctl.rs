use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("deployctl")
        .add_generator(
            "deploy_versions",
            Generator::script(
                CommandBuilder::single_command(
                    "curl -sL 'https://cdn.deno.land/deploy/meta/versions.json'",
                ),
                output_parsers::json_string_array,
            ),
        )
        .add_filter("filter-deployctl", template_filters::deployctl())
}
