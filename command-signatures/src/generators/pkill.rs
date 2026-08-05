use warp_completion_metadata::CommandSignatureGenerators;

use super::common;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pkill")
        .add_generator("process_name", common::process_names_generator())
        .add_generator("signal_name", common::signal_names_generator())
        .add_generator("user_name", common::users_generator())
}
