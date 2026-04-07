use warp_completion_metadata::CommandSignatureGenerators;

use crate::generators::common::{dependencies_generator, get_scripts_generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("bun")
        .add_generator("get_scripts_generator", get_scripts_generator())
        .add_generator("dependencies_generator", dependencies_generator())
}
