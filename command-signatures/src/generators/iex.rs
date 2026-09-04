use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("iex").add_filter("filter-exs", template_filters::exs())
}
