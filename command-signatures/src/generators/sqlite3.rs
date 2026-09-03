use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("sqlite3")
        .add_filter("filter-sqlite", template_filters::sqlite())
}
