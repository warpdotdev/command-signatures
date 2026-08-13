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
///
/// kubectl's flags are Cobra/pflag string flags, so a repeated flag's `Set` call overwrites the
/// previous value and kubectl itself acts on the *last* occurrence; this scans for the last match
/// to match that behavior. It also stops scanning at a bare `--` terminator, since pflag stops
/// parsing flags there and anything after it (including something that looks like a flag) is a
/// literal positional argument, not an option.
fn space_or_equals_delimited_option_value<'a>(
    tokens: &'a [&str],
    option_name: &str,
) -> Option<&'a str> {
    let option_name_equals = format!("{option_name}=");
    let scan_range = tokens
        .iter()
        .position(|token| *token == "--")
        .unwrap_or(tokens.len());
    let candidates = &tokens[..scan_range];
    let option_idx = candidates
        .iter()
        .rposition(|token| *token == option_name || token.starts_with(&option_name_equals));
    option_idx.and_then(|idx| {
        // This option is equals delimited, so position is option_name=value
        if let Some(equals_value) = candidates
            .get(idx)
            .and_then(|token| token.strip_prefix(&option_name_equals))
        {
            Some(equals_value)
        } else {
            // This option is space delimited, so value is the next token
            candidates.get(idx + 1).copied()
        }
    })
}

/// Whether `value` is made up only of characters that every shell these generator commands are
/// built for -- POSIX shells, PowerShell, and cmd.exe -- treats as ordinary literal text, so it
/// needs no quoting at all.
///
/// Real kubeconfig names sit comfortably inside this set: `minikube`, `docker-desktop`,
/// `gke_my-project_us-central1-a_prod`, `arn:aws:eks:us-east-1:1234:cluster/prod`,
/// `admin@prod.local`, and POSIX config paths like `/home/me/.kube/config`.
///
/// The set is deliberately conservative. Notably it excludes the backslash, which cmd.exe and
/// PowerShell treat literally but POSIX shells use as an escape, so a Windows-style path is
/// quoted rather than passed through.
fn is_safe_unquoted(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | ':' | '/' | '@' | '+')
        })
}

/// Renders `value` for safe embedding inside the single, unquoted command string built by
/// [`kubectl_script`].
///
/// A value that satisfies [`is_safe_unquoted`] -- which is every ordinary kubeconfig name -- is
/// interpolated bare, byte for byte as it was before any quoting existed here. That keeps the
/// generated command correct on POSIX shells, PowerShell and cmd.exe alike, since there is nothing
/// for any of them to interpret.
///
/// Anything else is wrapped in single quotes, with embedded single quotes escaped as `'\''` (end
/// the quoted string, emit an escaped literal quote, then reopen it) -- the same technique
/// `files_for_staging_command` in `git.rs` and the path prefix in `scp.rs` already use for this
/// class of hazard. This is what closes the injection hazard: a context named
/// `prod; touch /tmp/PWNED` cannot start a second command.
///
/// That quoted fallback is POSIX-shaped, and only POSIX-shaped, because there is no way to do
/// better from here: `kubectl_script`'s own `$KUBECONFIG` fallback below already requires POSIX
/// parameter expansion (`${VAR:+...}`), and `GeneratorProcess::CommandFromTokens` -- how every
/// caller reaches this code -- is never handed the runtime `Shell`, so the quoting style cannot be
/// selected per shell without threading `Shell` through that signature. Leaving the common case
/// unquoted confines that limitation to values that would otherwise be an injection vector,
/// instead of applying it to every kubectl completion on Windows.
fn escape_forwarded_value(value: &str) -> String {
    if is_safe_unquoted(value) {
        return value.to_owned();
    }
    format!("'{}'", value.replace('\'', r"'\''"))
}

/// Returns the value of a given `key` from a list of environment variables formatted as
/// `KEY=VALUE`.
fn env_var_value<'a>(env_vars: &'a [String], key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    env_vars.iter().find_map(|env| env.strip_prefix(&prefix))
}

/// Returns a command string to run the given `subcommand` string with the same `--namespace`,
/// `--context`, `--cluster`, `--user`, and/or `--kubeconfig` values as specified in the
/// incomplete command being entered (`tokens`), which scopes down suggestions to be more helpful
/// based on the already-specified namespace, context, cluster, user, or kubeconfig file. Also
/// reads the `KUBECONFIG` environment variable if `--kubeconfig` is not explicitly specified in
/// the tokens.
fn kubectl_script(
    env_vars: &[String],
    tokens: &[&str],
    subcommand: CommandBuilder,
) -> CommandBuilder {
    let kubeconfig_value = space_or_equals_delimited_option_value(tokens, "--kubeconfig")
        .or_else(|| env_var_value(env_vars, "KUBECONFIG"))
        .map(|value| format!("--kubeconfig={} ", escape_forwarded_value(value)))
        // Fall back to the $KUBECONFIG shell variable, which is set when session environment
        // variables are forwarded to the child process.
        .unwrap_or_else(|| r#"${KUBECONFIG:+--kubeconfig="$KUBECONFIG"} "#.to_owned());
    let context_value = space_or_equals_delimited_option_value(tokens, "--context")
        .map(|value| format!("--context={} ", escape_forwarded_value(value)))
        .unwrap_or_else(|| "".to_owned());
    let cluster_value = space_or_equals_delimited_option_value(tokens, "--cluster")
        .map(|value| format!("--cluster={} ", escape_forwarded_value(value)))
        .unwrap_or_else(|| "".to_owned());
    let user_value = space_or_equals_delimited_option_value(tokens, "--user")
        .map(|value| format!("--user={} ", escape_forwarded_value(value)))
        .unwrap_or_else(|| "".to_owned());
    let namespace_value = space_or_equals_delimited_option_value(tokens, "--namespace")
        .or(space_or_equals_delimited_option_value(tokens, "-n"))
        .map(|value| format!("--namespace={} ", escape_forwarded_value(value)))
        .unwrap_or_else(|| "".to_owned());

    let env_vars_str = env_vars.iter().join(" ");
    CommandBuilder::concat(
        CommandBuilder::single_command(format!(
            "{env_vars_str} kubectl {kubeconfig_value}{context_value}{cluster_value}{user_value}{namespace_value}"
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
    pub(super) static ref NAMESPACE_GENERATOR:Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("get namespace -o custom-columns=:.metadata.name")),
                |output| kubectl_post_process(output, None),
            );
    pub(super) static ref USER_GENERATOR: Generator =
            Generator::command_from_tokens(
                |tokens, _, env_vars| kubectl_script(env_vars, tokens, CommandBuilder::single_command("config get-users")),
                |output| match KubetctlStatus::from_output(output) {
                    KubetctlStatus::ConnectedToCluster | KubetctlStatus::GeneralError => {
                        GeneratorResults::default()
                    }
                    KubetctlStatus::Other => output
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty() && *line != "NAME")
                        .map(Suggestion::new)
                        .collect_unordered_results(),
                },
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
        .add_generator("namespace", NAMESPACE_GENERATOR.clone())
        .add_generator("user", USER_GENERATOR.clone())
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
            built.contains("--kubeconfig=/path/to/config"),
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
            built.contains("--kubeconfig=/tmp/kube-test/config"),
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
            built.contains("--kubeconfig=/flag/path/config"),
            "Expected --kubeconfig from flag (not env var), got: {built}"
        );
        assert!(
            !built.contains("--kubeconfig=/env/path/config"),
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
            built.contains("--namespace=kube-system"),
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
            built.contains("--namespace=kube-system"),
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
            built.contains("--namespace=kube-system"),
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
            built.contains("--context=staging-cluster"),
            "Expected --context=staging-cluster, got: {built}"
        );
        assert!(
            built.contains("--namespace=project1"),
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
            built.contains("--namespace=kube-system"),
            "Expected --namespace=kube-system from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_context_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context", "staging-cluster", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context=staging-cluster"),
            "Expected --context=staging-cluster from --context flag before subcommand, got: {built}"
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
            built.contains("--context=staging-cluster"),
            "Expected --context=staging-cluster from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_cluster_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--cluster", "prod-cluster", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--cluster=prod-cluster"),
            "Expected --cluster=prod-cluster from --cluster flag before subcommand, got: {built}"
        );
    }

    #[test]
    fn test_cluster_equals_syntax() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--cluster=prod-cluster", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--cluster=prod-cluster"),
            "Expected --cluster=prod-cluster from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_user_flag_before_subcommand() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--user", "jane-doe", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--user=jane-doe"),
            "Expected --user=jane-doe from --user flag before subcommand, got: {built}"
        );
    }

    #[test]
    fn test_user_equals_syntax() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--user=jane-doe", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--user=jane-doe"),
            "Expected --user=jane-doe from equals syntax, got: {built}"
        );
    }

    #[test]
    fn test_context_cluster_user_and_namespace_flags_all_forwarded() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--context",
            "staging-cluster",
            "--cluster",
            "prod-cluster",
            "--user",
            "jane-doe",
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
            built.contains("--context=staging-cluster"),
            "Expected --context=staging-cluster, got: {built}"
        );
        assert!(
            built.contains("--cluster=prod-cluster"),
            "Expected --cluster=prod-cluster, got: {built}"
        );
        assert!(
            built.contains("--user=jane-doe"),
            "Expected --user=jane-doe, got: {built}"
        );
        assert!(
            built.contains("--namespace=project1"),
            "Expected --namespace=project1, got: {built}"
        );
    }

    // --- escape_forwarded_value: the escaping hazard classes called out in review ---

    /// An ordinary value needs no quoting at all, so it is interpolated bare. This is the case that
    /// keeps the generated command correct on PowerShell and cmd.exe, where POSIX single quotes are
    /// not stripped: they never see a quote, because there is nothing here to quote.
    #[test]
    fn test_escape_forwarded_value_leaves_plain_value_unquoted() {
        assert_eq!(escape_forwarded_value("prod"), "prod");
    }

    /// The shapes real kubeconfig names take -- EKS ARNs, GKE names, POSIX paths, user@host -- all
    /// stay unquoted, so this is the path virtually every completion takes.
    #[test]
    fn test_escape_forwarded_value_leaves_realistic_names_unquoted() {
        for value in [
            "minikube",
            "docker-desktop",
            "gke_my-project_us-central1-a_prod",
            "arn:aws:eks:us-east-1:1234:cluster/prod",
            "admin@prod.local",
            "/home/me/.kube/config",
            "kube-system",
        ] {
            assert_eq!(
                escape_forwarded_value(value),
                value,
                "Expected `{value}` to need no quoting"
            );
        }
    }

    #[test]
    fn test_escape_forwarded_value_preserves_whitespace_as_a_single_argument() {
        assert_eq!(escape_forwarded_value("prod west"), "'prod west'");
    }

    #[test]
    fn test_escape_forwarded_value_escapes_embedded_single_quote() {
        assert_eq!(escape_forwarded_value("it's-prod"), r"'it'\''s-prod'");
    }

    #[test]
    fn test_escape_forwarded_value_prevents_variable_expansion() {
        assert_eq!(escape_forwarded_value("$HOME"), "'$HOME'");
    }

    #[test]
    fn test_escape_forwarded_value_prevents_command_separator_injection() {
        assert_eq!(escape_forwarded_value("prod; rm -rf /"), "'prod; rm -rf /'");
    }

    #[test]
    fn test_escape_forwarded_value_handles_backticks_and_double_quotes_literally() {
        assert_eq!(
            escape_forwarded_value(r#"`whoami`-"prod""#),
            r#"'`whoami`-"prod"'"#
        );
    }

    /// An empty value is quoted so it stays a present-but-empty argument rather than vanishing.
    #[test]
    fn test_escape_forwarded_value_quotes_empty_value() {
        assert_eq!(escape_forwarded_value(""), "''");
    }

    /// A Windows-style path contains a backslash, which POSIX shells would consume as an escape, so
    /// it falls into the quoted branch rather than being passed through bare.
    #[test]
    fn test_escape_forwarded_value_quotes_backslash() {
        assert_eq!(
            escape_forwarded_value(r"C:\Users\me\.kube\config"),
            r"'C:\Users\me\.kube\config'"
        );
    }

    // --- escape_forwarded_value flowing through kubectl_script's interpolation ---

    #[test]
    fn test_context_value_with_command_separator_is_quoted_not_executed() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context", "prod; rm -rf /", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context='prod; rm -rf /'"),
            "Expected the ';' to stay inside single quotes rather than starting a new command, got: {built}"
        );
    }

    #[test]
    fn test_namespace_value_with_embedded_quote_is_escaped() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--namespace", "it's-a-namespace", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains(r"--namespace='it'\''s-a-namespace'"),
            "Expected the embedded single quote to be escaped, got: {built}"
        );
    }

    #[test]
    fn test_kubeconfig_value_with_dollar_sign_is_not_expanded() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--kubeconfig",
            "$HOME/.kube/config",
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
            built.contains("--kubeconfig='$HOME/.kube/config'"),
            "Expected the $ to stay inside single quotes rather than being expanded, got: {built}"
        );
    }

    #[test]
    fn test_cluster_value_with_whitespace_stays_one_argument() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--cluster", "prod west", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--cluster='prod west'"),
            "Expected the quoted context name to remain a single argument, got: {built}"
        );
    }

    // --- space_or_equals_delimited_option_value: last-occurrence-wins and `--` termination ---

    #[test]
    fn test_space_or_equals_delimited_option_value_takes_last_occurrence() {
        let tokens = vec!["kubectl", "--context", "old", "--context", "new"];
        assert_eq!(
            space_or_equals_delimited_option_value(&tokens, "--context"),
            Some("new")
        );
    }

    #[test]
    fn test_space_or_equals_delimited_option_value_ignores_tokens_after_double_dash() {
        let tokens = vec!["kubectl", "get", "pods", "--", "--context", "fake"];
        assert_eq!(
            space_or_equals_delimited_option_value(&tokens, "--context"),
            None
        );
    }

    #[test]
    fn test_repeated_context_space_form_uses_last_occurrence() {
        let env_vars = vec![];
        let tokens = vec![
            "kubectl",
            "--context",
            "old",
            "--context",
            "new",
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
            built.contains("--context=new"),
            "Expected the last --context occurrence, matching kubectl's own flag-overwrite semantics, got: {built}"
        );
        assert!(
            !built.contains("--context=old"),
            "Did not expect the superseded --context value to be forwarded, got: {built}"
        );
    }

    #[test]
    fn test_repeated_context_equals_form_uses_last_occurrence() {
        let env_vars = vec![];
        let tokens = vec!["kubectl", "--context=old", "--context=new", "get", "pods"];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context=new"),
            "Expected the last --context=... occurrence, got: {built}"
        );
        assert!(
            !built.contains("--context=old"),
            "Did not expect the superseded --context value to be forwarded, got: {built}"
        );
    }

    #[test]
    fn test_flag_lookalike_after_double_dash_terminator_is_not_forwarded() {
        let env_vars = vec![];
        // pflag stops parsing flags at a bare `--`; a `--context`-looking token after it is a
        // literal positional argument, not a flag, and must not be forwarded.
        let tokens = vec![
            "kubectl",
            "--context",
            "real",
            "get",
            "pods",
            "--",
            "--context",
            "not-a-real-context",
        ];
        let cmd = kubectl_script(
            &env_vars,
            &tokens,
            CommandBuilder::single_command("get pods -o custom-columns=:.metadata.name"),
        );
        let built = cmd.build(Shell::Posix);
        assert!(
            built.contains("--context=real"),
            "Expected the --context before the `--` terminator to still be forwarded, got: {built}"
        );
        assert!(
            !built.contains("not-a-real-context"),
            "Did not expect the flag-lookalike after `--` to be forwarded, got: {built}"
        );
    }

    // --- USER_GENERATOR's output parser ---

    #[test]
    fn test_user_generator_filters_name_header() {
        let results = USER_GENERATOR.on_complete("NAME\nalice\nbob\n");
        let names: Vec<&str> = results
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert_eq!(
            names,
            vec!["alice", "bob"],
            "Expected the NAME header to be filtered out and only user names kept"
        );
    }

    #[test]
    fn test_user_generator_returns_empty_when_connected_to_cluster() {
        let results =
            USER_GENERATOR.on_complete("The connection to the server localhost:8080 was refused");
        assert!(
            results.suggestions.is_empty(),
            "Expected no suggestions when kubectl actually reached a server, got: {:?}",
            results.suggestions
        );
    }

    #[test]
    fn test_user_generator_returns_empty_on_general_error() {
        let results =
            USER_GENERATOR.on_complete("error: You must be logged in to the server (Unauthorized)");
        assert!(
            results.suggestions.is_empty(),
            "Expected no suggestions on a general kubectl error, got: {:?}",
            results.suggestions
        );
    }
}
