//! This module provides a mechanism to specify manually written information to merge into the
//! auto-generated command signatures. The data structures here mirror the ones in
//! [`warp_completion_metadata::fig_types`], but will contain a subset of the fields we care to
//! override. We may also have differences in invariants, e.g. optionality may differ.
use core::panic;
use std::{fs, io, path};

use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};
use warp_completion_metadata::fig_types::{Command, Template};

/// Contains hand-written information to be merged into an auto-generated command spec.
#[serde_as]
#[derive(Debug, Deserialize)]
struct CommandOverrides {
    #[serde(default)]
    pub options: Vec<OptionOverrides>,

    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<ArgOverrides>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct ArgOverrides {
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub template: Vec<Template>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct OptionOverrides {
    pub name: String,

    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<ArgOverrides>,
}

/// Check if this command has overrides defined. If there is no file for this command, return None.
fn get_overrides(name: &str) -> Option<CommandOverrides> {
    let overrides_path = path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("json")
        .join("overrides")
        .join("powershell")
        .join(format!("{}.json", name));

    match fs::File::open(overrides_path) {
        Ok(f) => {
            let overrides = serde_json::from_reader::<_, CommandOverrides>(io::BufReader::new(f))
                .unwrap_or_else(|err| panic!("failed to deserialize {name} overrides: {err:?}"));
            Some(overrides)
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => None,
            _ => panic!("Failed to read file: {}", e),
        },
    }
}

/// If this command has overrides defined, assign the data on the overrides onto the signature spec.
pub fn apply_overrides(command: &mut Command) -> Result<(), String> {
    let overrides = match command.name.as_slice() {
        [name] => get_overrides(name),
        names => {
            return Err(format!(
                "overrides not supported when names on a top-level command is {names:?}"
            ))
        }
    };
    let Some(overrides) = overrides else {
        return Ok(());
    };

    // Apply argument overrides by their position in the Vec.
    for (i, arg_overrides) in overrides.args.into_iter().enumerate() {
        if !arg_overrides.template.is_empty() {
            let arg = command.args.get_mut(i).ok_or(format!(
                "Tried to apply an override to positional argument at index {i}"
            ))?;
            arg.template = arg_overrides.template;
        }
    }

    // Option overrides are matched by name instead of position in the Vec.
    for option_override in overrides.options {
        let option = command
            .options
            .iter_mut()
            .find(|option| option.name.contains(&option_override.name))
            .ok_or(format!(
                "Tried to apply an override to option {}",
                option_override.name
            ))?;
        // Then, the arguments for the option are overwritten by position in Vec.
        for (i, arg_overrides) in option_override.args.into_iter().enumerate() {
            let arg = option.args.get_mut(i).ok_or(format!(
                "Tried to apply an override to argument {i} for option {}",
                option_override.name
            ))?;
            arg.template = arg_overrides.template;
        }
    }

    Ok(())
}
