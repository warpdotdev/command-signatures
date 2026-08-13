use itertools::Itertools;
use lazy_static::lazy_static;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, IconType, Suggestion,
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

/// Returns the value for the given `option_name`, which may be space delimited (--option value) or equals delimited (--option=value).
fn space_or_equals_delimited_option_value<'a>(
    tokens: &'a [&str],
    option_name: &str,
) -> Option<&'a str> {
    let option_name_equals = format!("{option_name}=");
    let option_idx = tokens
        .iter()
        .position(|token| *token == option_name || token.starts_with(&option_name_equals));
    option_idx.and_then(|idx| {
        // This option is equals delimited, so position is option_name=value
        if let Some(equals_value) = tokens
            .get(idx)
            .and_then(|token| token.strip_prefix(&option_name_equals))
        {
            Some(equals_value)
        } else {
            // This option is space delimited, so value is the next token
            tokens.get(idx + 1).copied()
        }
    })
}

/// Returns the value of a given `key` from a list of environment variables formatted as
/// `KEY=VALUE`.
fn env_var_value<'a>(env_vars: &'a [String], key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    env_vars.iter().find_map(|env| env.strip_prefix(&prefix))
}

/// Formats an option to forward into a generated command as `--option='value' `.
///
/// The value has to be quoted: tokens reach generators with their shell quoting already stripped,
/// so a value containing whitespace (e.g. a context named `my cluster`) would otherwise split into
/// multiple arguments. Single quotes with `'` escaped as `'\''` is the same technique `git.rs` and
/// `scp.rs` already use for interpolated token values, and unlike double quotes it also neutralizes
/// `$`, backticks and backslashes.
///
/// This is POSIX-only, which `kubectl_script` already is: its `$KUBECONFIG` fallback below uses
/// POSIX `${VAR:+...}` parameter expansion, and `GeneratorProcess::CommandFromTokens` does not pass
/// the runtime `Shell` down here, so there is no way to vary the quoting style per shell today.
fn forwarded_option(option_name: &str, value: &str) -> String {
    let escaped = value.replace('\'', r"'\''");
    format!("{option_name}='{escaped}' ")
}

/// Returns a command string to run the given `subcommand` string with the same `--kubeconfig`,
/// `--context`, `--cluster` and/or `--namespace` values as specified in the incomplete command
/// being entered (`tokens`), which scopes down suggestions to be more helpful based on the
/// already-specified cluster connection or namespace. Also reads the `KUBECONFIG` environment
/// variable if `--kubeconfig` is not explicitly specified in the tokens.
fn kubectl_script(
    env_vars: &[String],
    tokens: &[&str],
    subcommand: CommandBuilder,
) -> CommandBuilder {
    let kubeconfig_value = space_or_equals_delimited_option_value(tokens, "--kubeconfig")
        .or_else(|| env_var_value(env_vars, "KUBECONFIG"))
        .map(|value| forwarded_option("--kubeconfig", value))
        // Fall back to the $KUBECONFIG shell variable, which is set when session environment
        // variables are forwarded to the child process.
        .unwrap_or_else(|| r#"${KUBECONFIG:+--kubeconfig="$KUBECONFIG"} "#.to_owned());
    // `--context` and `--cluster` select which cluster the query runs against, so they must be
    // forwarded too: without them, every subsequent completion enumerates from the shell's active
    // context instead of the one written on the command line.
    let context_value = space_or_equals_delimited_option_value(tokens, "--context")
        .map(|value| forwarded_option("--context", value))
        .unwrap_or_default();
    let cluster_value = space_or_equals_delimited_option_value(tokens, "--cluster")
        .map(|value| forwarded_option("--cluster", value))
        .unwrap_or_default();
    let namespace_value = space_or_equals_delimited_option_value(tokens, "--namespace")
        .or(space_or_equals_delimited_option_value(tokens, "-n"))
        .map(|value| forwarded_option("--namespace", value))
        .unwrap_or_default();

    let env_vars_str = env_vars.iter().join(" ");
    CommandBuilder::concat(
        CommandBuilder::single_command(format!(
            "{env_vars_str} kubectl {kubeconfig_value}{context_value}{cluster_value}{namespace_value}"
        )),
        subcommand,
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

fn kubectl_builtin_complete_post_process(output: &str, icon: Option<IconType>) -> GeneratorResults {
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
            // Builtin completions output is already ordered semantically (e.g. pods on top, resource prefixes on bottom)
            .collect_ordered_results(),
    }
}

lazy_static! {
    pub(super) static ref RESOURCE_TYPE_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("api-resources -o name")),
        |output| kubectl_post_process(output, None),
    );
    pub(super) static ref RUNNING_PODS_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| {
            kubectl_script(
                env_vars,
                tokens,
                CommandBuilder::single_command("get pods --field-selector=status.phase=Running -o name"),
            )
        },
        |output| kubectl_post_process(output, Some(IconType::KubePod)),
    );
    pub(super) static ref DEPLOYMENTS_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| { kubectl_script(env_vars, tokens, CommandBuilder::single_command("get deployments -o custom-columns=:.metadata.name")) },
        |output| kubectl_post_process(output, None),
    );
    pub(super) static ref NODE_GENERATOR: Generator = Generator::command_from_tokens(
        |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("get nodes -o custom-columns=:.metadata.name")),
        |output| kubectl_post_process(output, None),
    );
    pub(super) static ref CLUSTER_ROLE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| {
                    kubectl_script(env_vars, tokens, CommandBuilder::single_command("get clusterroles -o custom-columns=:.metadata.name"))
                },
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref ROLE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("get roles -o custom-columns=:.metadata.name")),
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref RESOURCE_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, env_vars| {
                    // If there is trailing whitepsace, the last token is a resource type.
                    let resource_type = if has_trailing_whitespace {
                        tokens.last()
                    } else {
                        // If there is no trailing whitespace, the last token is a prefix of a resource name,
                        // and the token before is the resource type.
                        tokens.get(tokens.len().saturating_sub(2))
                    };
                    match resource_type {
                        Some(resource_type) => kubectl_script(
                            env_vars,
                            tokens,
                            CommandBuilder::single_command(format!("get {} -o custom-columns=:.metadata.name", resource_type)),
                        ),
                        None => CommandBuilder::single_command(""),
                    }
                },
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref CONTEXT_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("config get-contexts -o name")),
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref CLUSTER_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("config get-clusters")),
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
    pub(super) static ref USER_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("config get-users")),
                |output| match KubetctlStatus::from_output(output) {
                    KubetctlStatus::ConnectedToCluster | KubetctlStatus::GeneralError => {
                        GeneratorResults::default()
                    }
                    // `config get-users` has no `-o name` form, so its "NAME" header has to be
                    // filtered out of the suggestions.
                    KubetctlStatus::Other => output
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty() && *line != "NAME")
                        .map(Suggestion::new)
                        .collect_unordered_results(),
                },
            );
    pub(super) static ref NAMESPACE_GENERATOR:Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("get namespace -o custom-columns=:.metadata.name")),
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref TYPE_OR_TYPE_SLASH_NAME: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| {
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
                            env_vars,
                            tokens,
                            // Pipe to sed to add a {resource}/ prefix to every non empty line returned by the kubectl command.
                            // We need this prefix to match the last token in the input.
                            CommandBuilder::pipe(CommandBuilder::single_command(format!(r#"get {resource} -o custom-columns=:.metadata.name"#)), CommandBuilder::single_command(r#"sed '/./ s/^/{resource}\//'"#))
                        );
                    }
                    kubectl_script(env_vars, tokens, CommandBuilder::single_command("api-resources -o name"))
                },
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref KUBECTL_BUILTIN_COMPLETION: Generator =
    Generator::command_from_tokens(
        |tokens, has_trailing_whitespace, env_vars| {
            let env_vars_str = env_vars.iter().join(" ");
            let mut generation_command = vec![&env_vars_str, "kubectl", "__complete"].into_iter().chain(
                // Skip the first token which is just "kubectl"
                tokens.iter().skip(1).cloned()
            ).collect_vec();
            // The __complete command needs the empty string at the end
            if has_trailing_whitespace {
                generation_command.push("\"\"");
            }
            // Skip the last line since it is metadata, not a completion result.
            CommandBuilder::pipe(CommandBuilder::single_command(generation_command.join(" ")), CommandBuilder::single_command("sed '$d'"))

        },
        |output| kubectl_builtin_complete_post_process(output, None),
    );
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kubectl")
        .add_generator("resource_type", RESOURCE_TYPE_GENERATOR.clone())
        .add_generator("running_pods", RUNNING_PODS_GENERATOR.clone())
        .add_generator("deployments", DEPLOYMENTS_GENERATOR.clone())
        .add_generator("node", NODE_GENERATOR.clone())
        .add_generator("cluster_role", CLUSTER_ROLE_GENERATOR.clone())
        .add_generator("role", ROLE_GENERATOR.clone())
        .add_generator("resource", RESOURCE_GENERATOR.clone())
        .add_generator("context", CONTEXT_GENERATOR.clone())
        .add_generator("cluster", CLUSTER_GENERATOR.clone())
        .add_generator("user", USER_GENERATOR.clone())
        .add_generator("namespace", NAMESPACE_GENERATOR.clone())
        .add_generator("type_or_type_slash_name", TYPE_OR_TYPE_SLASH_NAME.clone())
        .add_generator(
            "kubectl_builtin_completion",
            KUBECTL_BUILTIN_COMPLETION.clone(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp_completion_metadata::Shell;

    #[test]
    fn test_kubeconfig_from_flag_in_tokens() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--kubeconfig",
            "/path/to/config",
            "config",
            "use-context",
        ];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("config get-contexts -o name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--kubeconfig='/path/to/config'"),
            "Expected --kubeconfig flag from tokens, got: {built}"
        );
    }

    #[test]
    fn test_kubeconfig_from_env_vars() {
        let env_vars = vec!["KUBECONFIG=/tmp/kube-test/config".to_string()];
        let tokens = vec!["kubectl", "config", "use-context"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("config get-contexts -o name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--kubeconfig='/tmp/kube-test/config'"),
            "Expected --kubeconfig from KUBECONFIG env var, got: {built}"
        );
    }

    #[test]
    fn test_kubeconfig_flag_takes_precedence_over_env_var() {
        let env_vars = vec!["KUBECONFIG=/env/path/config".to_string()];
        let tokens = vec![
            "kubectl",
            "--kubeconfig",
            "/flag/path/config",
            "config",
            "use-context",
        ];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("config get-contexts -o name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--kubeconfig='/flag/path/config'"),
            "Expected --kubeconfig from flag (not env var), got: {built}"
        );
        assert!(
            !built.contains("--kubeconfig='/env/path/config'"),
            "Should not contain env var value when flag is present, got: {built}"
        );
    }

    #[test]
    fn test_kubeconfig_fallback_to_shell_variable() {
        let env_vars: Vec<String> = vec![];
        let tokens = vec!["kubectl", "config", "use-context"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("config get-contexts -o name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("${KUBECONFIG:+--kubeconfig="),
            "Expected $KUBECONFIG shell variable fallback, got: {built}"
        );
    }

    #[test]
    fn test_env_var_value_finds_key() {
        let env_vars = vec!["FOO=bar".to_string(), "KUBECONFIG=/my/config".to_string()];
        assert_eq!(env_var_value(&env_vars, "KUBECONFIG"), Some("/my/config"));
        assert_eq!(env_var_value(&env_vars, "FOO"), Some("bar"));
        assert_eq!(env_var_value(&env_vars, "MISSING"), None);
    }

    #[test]
    fn test_namespace_short_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "-n", "kube-system", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--namespace='kube-system'"),
            "Expected --namespace=kube-system from -n flag before subcommand, got: {built}"
        );
    }

    #[test]
    fn test_namespace_long_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--namespace", "kube-system", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--namespace='kube-system'"),
            "Expected --namespace=kube-system from --namespace flag before subcommand, got: {built}"
        );
    }

    #[test]
    fn test_namespace_flag_after_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "get", "-n", "kube-system", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--namespace='kube-system'"),
            "Expected --namespace=kube-system from -n flag after subcommand, got: {built}"
        );
    }

    #[test]
    fn test_context_and_namespace_flags_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--context",
            "staging-cluster",
            "-n",
            "project1",
            "get",
            "pods",
        ];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='staging-cluster'"),
            "Expected --context=staging-cluster, got: {built}"
        );
        assert!(
            built.contains("--namespace='project1'"),
            "Expected --namespace=project1, got: {built}"
        );
    }

    #[test]
    fn test_namespace_equals_syntax() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--namespace=kube-system", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--namespace='kube-system'"),
            "Expected --namespace=kube-system from equals syntax, got: {built}"
        );
    }

    /// The whole generated command for the case reported in warpdotdev/warp#5186 and
    /// warpdotdev/warp#3929: completing `--namespace` after a `--context` has been written on the
    /// line must query the cluster named by that context, not the shell's active one.
    #[test]
    fn test_full_generated_command_forwards_context_to_namespace_query() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context", "staging-cluster", "--namespace"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get namespace -o custom-columns=:.metadata.name"),
        );
        assert_eq!(
            cmd.build(Shell::Posix),
            concat!(
                r#" kubectl ${KUBECONFIG:+--kubeconfig="$KUBECONFIG"} "#,
                "--context='staging-cluster'  ",
                "get namespace -o custom-columns=:.metadata.name",
            )
        );
    }

    #[test]
    fn test_context_long_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context", "staging-cluster", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='staging-cluster'"),
            "Expected --context=staging-cluster from --context flag before subcommand, got: {built}"
        );
    }

    #[test]
    fn test_context_flag_after_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "get", "--context", "staging-cluster", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='staging-cluster'"),
            "Expected --context=staging-cluster from --context flag after subcommand, got: {built}"
        );
    }

    #[test]
    fn test_context_equals_syntax() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context=staging-cluster", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='staging-cluster'"),
            "Expected --context=staging-cluster from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_cluster_flag_forwarded() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--cluster", "staging", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--cluster='staging'"),
            "Expected --cluster=staging from --cluster flag, got: {built}"
        );
    }

    #[test]
    fn test_cluster_equals_syntax() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--cluster=staging", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--cluster='staging'"),
            "Expected --cluster=staging from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_no_flags_forwarded_when_absent() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            !built.contains("--context"),
            "Did not expect a --context flag, got: {built}"
        );
        assert!(
            !built.contains("--cluster"),
            "Did not expect a --cluster flag, got: {built}"
        );
        assert!(
            !built.contains("--namespace"),
            "Did not expect a --namespace flag, got: {built}"
        );
    }

    /// Tokens reach generators with their shell quoting stripped, so values containing whitespace
    /// have to be re-quoted or they would split into separate arguments.
    #[test]
    fn test_forwarded_values_with_whitespace_are_quoted() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--context",
            "my staging cluster",
            "--namespace",
            "my namespace",
            "get",
            "pods",
        ];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='my staging cluster'"),
            "Expected a quoted --context value, got: {built}"
        );
        assert!(
            built.contains("--namespace='my namespace'"),
            "Expected a quoted --namespace value, got: {built}"
        );
    }

    /// A literal single quote in a value has to be escaped, or it would close the quoted string
    /// and let the rest of the value be interpreted by the shell.
    #[test]
    fn test_forwarded_value_with_single_quote_is_escaped() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context", "it's-staging", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains(r#"--context='it'\''s-staging'"#),
            "Expected an escaped single quote in the --context value, got: {built}"
        );
    }

    /// Values are quoted rather than interpolated raw, so shell metacharacters in a value cannot
    /// be interpreted by the shell that runs the generated command.
    #[test]
    fn test_forwarded_value_does_not_expand_shell_metacharacters() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context=$(id)", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='$(id)'"),
            "Expected the value to stay inside single quotes, got: {built}"
        );
    }
}
