use std::process;

use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_with::{formats::PreferMany, serde_as, DefaultOnNull, OneOrMany};

#[serde_as]
#[derive(Debug, Deserialize)]
struct CmdletHelp {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Synopsis")]
    synopsis: String,
    #[serde(alias = "ModuleName")]
    module_name: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    description: Vec<Paragraph>,
    #[serde(deserialize_with = "empty_string_is_none")]
    parameters: Option<ParameterTypes>,
    #[serde(alias = "Syntax")]
    syntax: SyntaxInfo,
}

#[derive(Debug, Deserialize)]
struct Paragraph {
    #[serde(alias = "Text")]
    text: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct ParameterTypes {
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    parameter: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
struct Parameter {
    name: String,
    #[serde(rename = "type")]
    type_info: TypeInfo,
    #[serde(default)]
    description: Vec<Paragraph>,
    #[serde(default, rename = "parameterValueGroup")]
    allowed_values: Option<ParameterValues>,
    #[serde(default, rename = "defaultValue")]
    default_value: Option<String>,
    required: String,
    #[serde(default, rename = "variableLength")]
    variable_length: Option<String>,
    globbing: String,
    #[serde(rename = "pipelineInput")]
    pipeline_input: String,
    position: String,
    aliases: String,
}

#[derive(Debug, Deserialize)]
struct TypeInfo {
    name: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct ParameterValues {
    #[serde(rename = "parameterValue")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    values: Vec<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct SyntaxTypes {
    name: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    parameter: Vec<Parameter>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct SyntaxInfo {
    #[serde(rename = "syntaxItem")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    syntax_items: Vec<SyntaxTypes>,
}

fn main() {
    let all_cmdlet_names =
        run_pwsh_command("Get-Command -Type Cmdlet | Select-Object -ExpandProperty Name");
    let all_cmdlet_names = all_cmdlet_names.trim().split('\n').collect_vec();
    let all_cmdlet_help = all_cmdlet_names
        .par_iter()
        .map(|cmdlet_name| {
            let cmdlet_help_json =
                run_pwsh_command(format!("Get-Help {cmdlet_name} | ConvertTo-Json -Depth 8"));
            serde_json::from_str::<CmdletHelp>(&cmdlet_help_json)
                .expect(&format!("failed to deserialize {cmdlet_name} help"))
        })
        .collect::<Vec<_>>();
    let dbg_index = all_cmdlet_names
        .iter()
        .find_position(|name| **name == "Get-Command");
    if let Some((i, _)) = dbg_index {
        dbg!(&all_cmdlet_help[i]);
    }
    for cmdlet_help in all_cmdlet_help {
        println!("{}", cmdlet_help.name);
        println!(
            "{}",
            cmdlet_help
                .syntax
                .syntax_items
                .iter()
                .map(|item| item.parameter.len())
                .sum::<usize>()
        );
        println!(
            "{:?}",
            cmdlet_help.parameters.map(|param| param.parameter.len())
        );
    }
}

fn run_pwsh_command<S: AsRef<str>>(command: S) -> String {
    let stdout_bytes = process::Command::new("pwsh")
        .args(["-NoProfile", "-NoLogo", "-Command", command.as_ref()])
        .output()
        .expect("pwsh must be installed")
        .stdout;
    String::from_utf8(stdout_bytes).expect("pwsh output must be valid UTF8")
}

/// Sometimes an empty string is placed in a field which is an object type. This will convert that
/// to a `None`.
fn empty_string_is_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value == Value::String("".to_string()) {
        return Ok(None);
    }
    T::deserialize(value)
        .map(|v| Some(v))
        .map_err(serde::de::Error::custom)
}
