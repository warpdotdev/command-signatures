use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("n")
        .add_generator(
            "lsr",
            Generator::script(
                CommandBuilder::single_command("n lsr --all"),
                output_parsers::slice2_reversed,
            ),
        )
        .add_filter("filter-js-ts-family", template_filters::js_ts_family())
}
