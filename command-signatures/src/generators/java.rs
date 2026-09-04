use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("java")
        .add_filter("filter-java-class", template_filters::java_class())
        .add_filter("filter-jar", template_filters::jar())
}
