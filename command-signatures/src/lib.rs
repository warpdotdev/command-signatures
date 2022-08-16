mod generators;

pub use all_commands::*;
pub use generators::generators;

pub use warp_completion_metadata::{
    Argument, ArgumentType, CommandGenerators, FilterTemplateSuggestion, Filters, Generator,
    GeneratorName, GeneratorProcess, GeneratorResults, GeneratorResultsCollector, Generators,
    Importance, IsArgumentOptional, Opt, Order, PathSuggestionType, Priority, Signature,
    Suggestion, Template, TemplateFilter, TemplateFilters, TemplateType,
};

#[cfg(not(feature = "new_fig_specs"))]
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

#[cfg(feature = "new_fig_specs")]
pub fn commands() -> Vec<Signature> {
    new_command_signatures_1::commands::signatures()
        .into_iter()
        .chain(new_command_signatures_2::signatures().into_iter())
        .chain(new_command_signatures_3::signatures().into_iter())
        .chain(new_command_signatures_4::signatures().into_iter())
        .chain(new_command_signatures_5::signatures().into_iter())
        .chain(new_command_signatures_6::signatures().into_iter())
        .collect()
}

#[cfg(not(new_fig_specs))]
mod all_commands {
    pub use command_signatures_1::commands::*;
    pub use command_signatures_2::commands::*;
    pub use command_signatures_3::commands::*;
    pub use command_signatures_4::commands::*;
    pub use command_signatures_5::commands::*;
    pub use command_signatures_6::commands::*;
}

#[cfg(new_fig_specs)]
mod all_commands {
    pub use new_command_signatures_1::commands::*;
    pub use new_command_signatures_2::commands::*;
    pub use new_command_signatures_3::commands::*;
    pub use new_command_signatures_4::commands::*;
    pub use new_command_signatures_5::commands::*;
    pub use new_command_signatures_6::commands::*;
}
