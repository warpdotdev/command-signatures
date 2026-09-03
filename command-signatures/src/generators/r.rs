use super::{output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("r")
        .add_generator(
            "rscript_libpaths",
            Generator::script(
                CommandBuilder::single_command("Rscript -e '.libPaths()'"),
                output_parsers::named_lines,
            ),
        )
        .add_filter("filter-r", template_filters::r())
        .add_filter("filter-rd", template_filters::rd())
        .add_filter("filter-r-src", template_filters::r_src())
        .add_filter("filter-r-archive", template_filters::r_archive())
}
