use super::template_filters;
use warp_completion_metadata::CommandSignatureGenerators;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("wasm-bindgen")
        .add_filter("filter-wasm", template_filters::wasm())
}
