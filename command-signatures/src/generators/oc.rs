//! `oc` is the OpenShift CLI. It is a superset of `kubectl` (both Cobra-based, same `__complete`
//! protocol), so its generators mirror kubectl's but invoke the `oc` binary directly.
use itertools::Itertools;
use lazy_static::lazy_static;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, IconType, Suggestion,
};

use super::kubectl::{
    kube_cli_builtin_complete_post_process, kube_cli_post_process, kube_cli_script, KubetctlStatus,
};

fn oc_script(env_vars: &[String], tokens: &[&str], subcommand: CommandBuilder) -> CommandBuilder {
    kube_cli_script("oc", env_vars, tokens, subcommand)
}

lazy_static! {
    static ref RESOURCE_TYPE_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("api-resources -o name")),
        |output| kube_cli_post_process(output, None),
    );
    static ref RUNNING_PODS_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| {
            oc_script(
                env_vars,
                tokens,
                CommandBuilder::single_command("get pods --field-selector=status.phase=Running -o name"),
            )
        },
        |output| kube_cli_post_process(output, Some(IconType::KubePod)),
    );
    static ref DEPLOYMENTS_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| { oc_script(env_vars, tokens, CommandBuilder::single_command("get deployments -o custom-columns=:.metadata.name")) },
        |output| kube_cli_post_process(output, None),
    );
    static ref NODE_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("get nodes -o custom-columns=:.metadata.name")),
        |output| kube_cli_post_process(output, None),
    );
    static ref CLUSTER_ROLE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| {
                    oc_script(env_vars, tokens, CommandBuilder::single_command("get clusterroles -o custom-columns=:.metadata.name"))
                },
                |output| kube_cli_post_process(output, None),
            );
    static ref ROLE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("get roles -o custom-columns=:.metadata.name")),
                |output| kube_cli_post_process(output, None),
            );
    static ref RESOURCE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, env_vars| {
                    let resource_type = if has_trailing_whitespace {
                        tokens.last()
                    } else {
                        tokens.get(tokens.len().saturating_sub(2))
                    };
                    match resource_type {
                        Some(resource_type) => oc_script(
                            env_vars,
                            tokens,
                            CommandBuilder::single_command(format!("get {} -o custom-columns=:.metadata.name", resource_type)),
                        ),
                        None => CommandBuilder::single_command(""),
                    }
                },
                |output| kube_cli_post_process(output, None),
            );
    static ref CONTEXT_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("config get-contexts -o name")),
                |output| kube_cli_post_process(output, None),
            );
    static ref CLUSTER_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("config get_clusters")),
                |output| match KubetctlStatus::from_output(output) {
                    KubetctlStatus::ConnectedToCluster | KubetctlStatus::GeneralError => {
                        GeneratorResults::default()
                    }
                    KubetctlStatus::Other => output
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty() && *line != "NAME")
                        .map(|name| Suggestion::new(name).with_icon(IconType::KubeCluster))
                        .collect_unordered_results(),
                },
            );
    static ref NAMESPACE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| oc_script(env_vars, tokens, CommandBuilder::single_command("get namespace -o custom-columns=:.metadata.name")),
                |output| kube_cli_post_process(output, None),
            );
    static ref TYPE_OR_TYPE_SLASH_NAME: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| {
                    if let Some(resource) = tokens.last().and_then(|last_input| {
                        last_input.find('/').map(|index| &last_input[0..index])
                    }) {
                        return oc_script(
                            env_vars,
                            tokens,
                            CommandBuilder::pipe(CommandBuilder::single_command(format!(r#"get {resource} -o custom-columns=:.metadata.name"#)), CommandBuilder::single_command(r#"sed '/./ s/^/{resource}\\/'"#))
                        );
                    }
                    oc_script(env_vars, tokens, CommandBuilder::single_command("api-resources -o name"))
                },
                |output| kube_cli_post_process(output, None),
            );
    static ref OC_BUILTIN_COMPLETION: Generator =
    Generator::command_from_tokens(
        |tokens, has_trailing_whitespace, env_vars| {
            let env_vars_str = env_vars.iter().join(" ");
            let mut generation_command = vec![&env_vars_str, "oc", "__complete"].into_iter().chain(
                // Skip the first token which is just "oc"
                tokens.iter().skip(1).cloned()
            ).collect_vec();
            if has_trailing_whitespace {
                generation_command.push("\"\"");
            }
            // Skip the last line since it is metadata, not a completion result.
            CommandBuilder::pipe(CommandBuilder::single_command(generation_command.join(" ")), CommandBuilder::single_command("sed '$d'"))
        },
        |output| kube_cli_builtin_complete_post_process(output, None),
    );
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("oc")
        .add_generator("resource_type", RESOURCE_TYPE_GENERATOR.clone())
        .add_generator("running_pods", RUNNING_PODS_GENERATOR.clone())
        .add_generator("deployments", DEPLOYMENTS_GENERATOR.clone())
        .add_generator("node", NODE_GENERATOR.clone())
        .add_generator("cluster_role", CLUSTER_ROLE_GENERATOR.clone())
        .add_generator("role", ROLE_GENERATOR.clone())
        .add_generator("resource", RESOURCE_GENERATOR.clone())
        .add_generator("context", CONTEXT_GENERATOR.clone())
        .add_generator("cluster", CLUSTER_GENERATOR.clone())
        .add_generator("namespace", NAMESPACE_GENERATOR.clone())
        .add_generator("type_or_type_slash_name", TYPE_OR_TYPE_SLASH_NAME.clone())
        .add_generator("oc_builtin_completion", OC_BUILTIN_COMPLETION.clone())
}
