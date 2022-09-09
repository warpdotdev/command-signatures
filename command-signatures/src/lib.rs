mod generators;

pub use command_signatures_1::commands::*;
pub use command_signatures_2::commands::*;
pub use command_signatures_3::commands::*;
pub use command_signatures_4::commands::*;
pub use command_signatures_5::commands::*;
pub use command_signatures_6::commands::*;
pub use generators::command_signature_generators;

pub use warp_completion_metadata::*;

pub fn commands() -> Vec<Signature> {
    command_signatures_1::commands::signatures()
        .into_iter()
        .chain(command_signatures_2::signatures().into_iter())
        .chain(command_signatures_3::signatures().into_iter())
        .chain(command_signatures_4::signatures().into_iter())
        .chain(command_signatures_5::signatures().into_iter())
        .chain(command_signatures_6::signatures().into_iter())
        .collect()
}
