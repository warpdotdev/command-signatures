//! This module provides a mechanism to specify manually written information to merge into the
//! auto-generated command signatures. The data structures here mirror the ones in
//! [`warp_completion_metadata::fig_types`], but will contain a subset of the fields we care to
//! override. We may also have differences in invariants, e.g. optionality may differ.
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};
use warp_completion_metadata::fig_types::Template;

/// Contains hand-written information to be merged into an auto-generated command spec.
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CommandOverrides {
    #[serde(default)]
    pub options: Vec<OptionOverrides>,

    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<ArgOverrides>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ArgOverrides {
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub template: Vec<Template>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct OptionOverrides {
    pub name: String,

    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub args: Vec<ArgOverrides>,
}
