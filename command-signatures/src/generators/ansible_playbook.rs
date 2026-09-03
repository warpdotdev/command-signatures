use super::fig_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ansible-playbook")
        .add_filter("filter-yml-yaml", fig_filters::yml_yaml())
}
