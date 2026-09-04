use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("direnv").add_filter("filter-envrc", template_filters::envrc())
}
