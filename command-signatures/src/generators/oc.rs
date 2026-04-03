//! `oc` is the OpenShift CLI. It shares most of its Kubernetes-related functionality with kubectl
//! (both are Cobra-based) but uses `oc` commands instead of `kubectl` for proper authentication
//! context, and adds OpenShift-specific generators for projects and build configs.
use itertools::Itertools;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use super::kubectl::{
    CLUSTER_GENERATOR, CLUSTER_ROLE_GENERATOR, CONTEXT_GENERATOR, DEPLOYMENTS_GENERATOR,
    NAMESPACE_GENERATOR, NODE_GENERATOR, RESOURCE_GENERATOR, RESOURCE_TYPE_GENERATOR,
    ROLE_GENERATOR, RUNNING_PODS_GENERATOR, TYPE_OR_TYPE_SLASH_NAME,
};

fn oc_post_process(output: &str) -> GeneratorResults {
    if output.contains("error:") || output.contains("The connection to the server") {
        return GeneratorResults::default();
    }
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Suggestion::new)
        .collect_unordered_results()
}

fn oc_builtin_complete_post_process(output: &str) -> GeneratorResults {
    if output.contains("error:") || output.contains("The connection to the server") {
        return GeneratorResults::default();
    }
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Suggestion::new)
        .collect_ordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    let projects_generator = Generator::script(
        CommandBuilder::single_command("oc projects -q"),
        oc_post_process,
    );

    let build_configs_generator = Generator::script(
        CommandBuilder::single_command("oc get buildconfigs -o custom-columns=:.metadata.name"),
        oc_post_process,
    );

    let oc_builtin_completion = Generator::command_from_tokens(
        |tokens, has_trailing_whitespace, env_vars| {
            let env_vars_str = env_vars.iter().join(" ");
            let mut generation_command = vec![&env_vars_str, "oc", "__complete"]
                .into_iter()
                .chain(
                    // Skip the first token which is just "oc"
                    tokens.iter().skip(1).cloned(),
                )
                .collect_vec();
            // The __complete command needs the empty string at the end
            if has_trailing_whitespace {
                generation_command.push("\"\"");
            }
            // Skip the last line since it is metadata, not a completion result.
            CommandBuilder::pipe(
                CommandBuilder::single_command(generation_command.join(" ")),
                CommandBuilder::single_command("sed '$d'"),
            )
        },
        oc_builtin_complete_post_process,
    );

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
        .add_generator("oc_builtin_completion", oc_builtin_completion)
        .add_generator("projects", projects_generator)
        .add_generator("build_configs", build_configs_generator)
}
