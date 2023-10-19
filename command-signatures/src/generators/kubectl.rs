use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, IconType,
    Suggestion,
};

enum KubetctlStatus {
    ConnectedToCluster,
    Other,
    GeneralError,
}

impl KubetctlStatus {
    fn from_output(output: &str) -> Self {
        if output.contains("The connection to the server") {
            KubetctlStatus::ConnectedToCluster
        } else if output.contains("error:") {
            KubetctlStatus::GeneralError
        } else {
            KubetctlStatus::Other
        }
    }
}

/// Returns the value for the given `option_name`, which may only be space delimited.
fn space_delimited_option_value<'a>(tokens: &'a [&str], option_name: &str) -> Option<&'a str> {
    let option_idx = tokens.iter().position(|token| *token == option_name);
    option_idx.and_then(|idx| tokens.get(idx + 1).copied())
}

/// Returns a command string to run the given `subcommand` string with the same `--namespace` and/or
/// `--kubeconfig` values as specified in the incomplete command being entered (`tokens`), which
/// scopes down suggestions to be more helpful based on the already-specified namespace or
/// kubeconfig file.
fn kubectl_script(tokens: &[&str], subcommand: impl AsRef<str>) -> String {
    let kubeconfig_value = space_delimited_option_value(tokens, "--kubeconfig")
        .map(|value| format!("--kubeconfig={value} "))
        .unwrap_or_else(|| "".to_owned());
    let namespace_value = space_delimited_option_value(tokens, "--namespace")
        .or(space_delimited_option_value(tokens, "-n"))
        .map(|value| format!("--namespace={value} "))
        .unwrap_or_else(|| "".to_owned());

    format!(
        "kubectl {kubeconfig_value}{namespace_value}{}",
        subcommand.as_ref()
    )
}

fn kubectl_post_process(output: &str, icon: Option<IconType>) -> GeneratorResults {
    match KubetctlStatus::from_output(output) {
        KubetctlStatus::ConnectedToCluster | KubetctlStatus::GeneralError => {
            GeneratorResults::default()
        }
        KubetctlStatus::Other => output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|suggestion| match icon {
                Some(icon) => Suggestion::new(suggestion).with_icon(icon),
                None => Suggestion::new(suggestion),
            })
            .collect_unordered_results(),
    }
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kubectl")
        .add_generator(
            "resource_type",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "api-resources -o name"),
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "running_pods",
            Generator::command_from_tokens(
                |tokens| {
                    kubectl_script(
                        tokens,
                        "get pods --field-selector=status.phase=Running -o name",
                    )
                },
                |output| kubectl_post_process(output, Some(IconType::KubePod)),
            ),
        )
        .add_generator(
            "deployments",
            Generator::command_from_tokens(
                |tokens| {
                    kubectl_script(tokens, "get deployments -o custom-columns=:.metadata.name")
                },
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "node",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "get nodes -o custom-columns=:.metadata.name"),
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "cluster_role",
            Generator::command_from_tokens(
                |tokens| {
                    kubectl_script(tokens, "get clusterroles -o custom-columns=:.metadata.name")
                },
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "role",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "get roles -o custom-columns=:.metadata.name"),
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "resource",
            Generator::command_from_tokens(
                |tokens| match tokens.last() {
                    Some(type_name) => kubectl_script(
                        tokens,
                        format!("get {} -o custom-columns=:.metadata.name", type_name),
                    ),
                    None => "".to_string(),
                },
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "context",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "config get-contexts -o name"),
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "cluster",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "config get_clusters"),
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
            ),
        )
        .add_generator(
            "namespace",
            Generator::command_from_tokens(
                |tokens| kubectl_script(tokens, "get namespace -o custom-columns=:.metadata.name"),
                |output| kubectl_post_process(output, None),
            ),
        )
        .add_generator(
            "type_or_type_slash_name",
            Generator::command_from_tokens(
                |tokens| {
                    // This is not correct (Fig's implementation is broken too). The last token
                    // might not be a an incomplete resource type/name token; it could be the value
                    // for an option. So, for example, if you specified a value for '--kubeconfig'
                    // (which is a path and likely to include '/'), this mistakenly assumes that
                    // path value is an incomplete resource type/name.
                    //
                    // The logic here really should be actually parsing the tokens into
                    // options/arguments to determine how the resource type/name should be
                    // completed.
                    if let Some(resource) = tokens.last().and_then(|last_input| {
                        last_input.find('/').map(|index| &last_input[0..index])
                    }) {
                        return kubectl_script(
                            tokens,
                            format!("get {} -o custom-columns=:.metadata.name", resource),
                        );
                    }
                    kubectl_script(tokens, "api-resources -o name")
                },
                |output| kubectl_post_process(output, None),
            ),
        )
}
