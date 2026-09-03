use super::fig_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("direnv").add_filter("filter-envrc", fig_filters::envrc())
}
