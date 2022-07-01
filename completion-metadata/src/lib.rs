pub mod fig_types;
pub mod rust_generator;
mod signature;

use serde::Serialize;
pub use signature::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct Suggestion {
    pub exact_string: String,
    pub description: Option<String>,
}

impl Suggestion {
    pub fn new(name: impl Into<String>) -> Self {
        Suggestion {
            exact_string: name.into(),
            description: None,
        }
    }

    pub fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
        Suggestion {
            exact_string: name.into(),
            description: Some(description.into()),
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Suggestion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exact_string.hash(state)
    }
}

pub type Generators = HashMap<GeneratorName, Generator>;

#[derive(Clone)]
pub struct CommandGenerators {
    command_name: String,
    generators: Generators,
}

impl From<CommandGenerators> for (String, Generators) {
    fn from(command_generators: CommandGenerators) -> Self {
        (
            command_generators.command_name,
            command_generators.generators,
        )
    }
}

impl CommandGenerators {
    pub fn new(command_name: impl Into<String>) -> Self {
        Self {
            command_name: command_name.into(),
            generators: HashMap::new(),
        }
    }

    pub fn add_generator(
        mut self,
        generator_name: impl Into<GeneratorName>,
        generator: Generator,
    ) -> Self {
        self.generators.insert(generator_name.into(), generator);
        self
    }

    pub fn generators(&self) -> &Generators {
        &self.generators
    }
}
