use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rollup")
        .add_filter("filter-config-js", template_filters::config_js())
}
