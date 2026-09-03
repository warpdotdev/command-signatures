use super::fig_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ts-node")
        .add_filter("filter-tsconfig", fig_filters::tsconfig_json())
}
