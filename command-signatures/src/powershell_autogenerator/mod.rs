//! A note about `String` vs `&str` when using serde_json...
//! Although often times using `&str` is possible, even desirable for achieving zero-copy
//! deserialization, it isn't so easy in our case with JSON. JSON values may contain values escaped
//! with backslashes, e.g. double-quotes, newlines, etc. When those are encountered, serde is unable
//! to hand out a `&str` to the data directly, as it must _decode_ the escaped values, e.g. convert
//! "\\" "n" into the actual newline character, byte `10`, and return that as owned data. Therefore,
//! using `&str` for a field which has escaped values will actually cause a deserialization failure.
//! The 2 ways around that are to use a `String` as I've done here, which simply always copies the
//! data, or to use `Cow<'_, str>`. Since this for a script we'll run during development, and not
//! during Warp runtime, a few extra copies don't really matter. It makes the code simpler. So, I've
//! gone with `String`.
mod deserializers;
mod to_fig_types;

use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, DefaultOnNull, OneOrMany};

use deserializers::*;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CmdletHelp {
    #[serde(alias = "Name")]
    pub name: String,

    #[serde(alias = "Synopsis")]
    pub synopsis: String,

    /// Doesn't come from `Get-Help`. Need to separately get it from `Get-Alias`.
    #[serde(skip_deserializing)]
    pub aliases: Vec<String>,

    #[allow(dead_code)]
    #[serde(alias = "ModuleName")]
    pub module_name: String,

    #[allow(dead_code)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: Vec<Paragraph>,

    #[serde(deserialize_with = "empty_string_is_none")]
    pub parameters: Option<ParameterTypes>,

    #[serde(alias = "Syntax")]
    pub syntax: SyntaxInfo,
}

#[derive(Debug, Deserialize)]
pub struct Paragraph {
    #[serde(alias = "Text")]
    pub text: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Default)]
pub struct ParameterTypes {
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub parameter: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,

    #[serde(rename = "type")]
    pub type_info: TypeInfo,

    #[serde(default)]
    pub description: Vec<Paragraph>,

    #[serde(default, rename = "parameterValueGroup")]
    pub allowed_values: Option<ParameterValues>,

    #[serde(
        default,
        rename = "defaultValue",
        deserialize_with = "literal_none_is_none"
    )]
    pub default_value: Option<String>,

    #[serde(deserialize_with = "string_to_bool")]
    pub required: bool,

    #[serde(
        default,
        rename = "variableLength",
        deserialize_with = "string_to_bool"
    )]
    pub variable_length: bool,

    #[serde(default, deserialize_with = "string_to_bool")]
    pub globbing: bool,

    /// Possible values: "False", "True", "True (ByValue)", "True (ByPropertyName)",
    /// "True (ByPropertyName, ByValue)"
    #[serde(rename = "pipelineInput")]
    pub pipeline_input: String,

    /// Possible values: "named", "0", "1", "2", "3", "100", "101"
    /// The "100" and "101" values seem like errors, and are observed in the `Register-EngineEvent`
    /// and `Register-ObjectEvent` cmdlets on pwsh 7.4.2.
    pub position: String,

    #[serde(deserialize_with = "literal_none_is_empty")]
    pub aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypeInfo {
    pub name: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ParameterValues {
    #[serde(rename = "parameterValue")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub values: Vec<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SyntaxTypes {
    #[allow(dead_code)]
    pub name: String,

    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub parameter: Vec<Parameter>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SyntaxInfo {
    #[serde(rename = "syntaxItem")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub syntax_items: Vec<SyntaxTypes>,
}
