use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("python").add_filter("filter-py", template_filters::py())
}

pub fn python3_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("python3").add_filter("filter-py", template_filters::py())
}
