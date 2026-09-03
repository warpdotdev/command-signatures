use crate::fig_types::Command;
use std::collections::HashSet;
use std::fmt;

/// How to treat a `loadSpec` target that cannot be resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissingLoadSpecPolicy {
    /// Leave the wrapper command unchanged (aside from clearing `load_spec`).
    Skip,
    /// Return [`LoadSpecError::Missing`].
    Error,
}

/// Failure while resolving a static `loadSpec` reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadSpecError {
    Missing { from: String, target: String },
    Cycle { stack: Vec<String> },
}

impl fmt::Display for LoadSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadSpecError::Missing { from, target } => {
                write!(
                    f,
                    "loadSpec {target:?} referenced from {from:?} was not found"
                )
            }
            LoadSpecError::Cycle { stack } => {
                write!(f, "loadSpec cycle: {}", stack.join(" -> "))
            }
        }
    }
}

/// Looks up a command spec by the `loadSpec` target name (for example `"flutter"`).
pub trait SpecLookup {
    fn get(&self, name: &str) -> Option<Command>;
}

impl<F> SpecLookup for F
where
    F: Fn(&str) -> Option<Command>,
{
    fn get(&self, name: &str) -> Option<Command> {
        self(name)
    }
}

/// Resolves every static string `loadSpec` in `command` and its subcommands.
///
/// Wrapper `name` / `description` / `priority` / parser directives are kept. The
/// target fills in empty `args` / `options` / `subcommands`; otherwise the
/// wrapper's lists are concatenated in front of the target's.
pub fn resolve_load_specs(
    command: Command,
    lookup: &impl SpecLookup,
    missing: MissingLoadSpecPolicy,
) -> Result<Command, LoadSpecError> {
    resolve_tree(command, lookup, &mut Vec::new(), missing)
}

fn command_label(command: &Command) -> String {
    command
        .name
        .first()
        .cloned()
        .unwrap_or_else(|| "<unnamed>".to_owned())
}

fn resolve_tree(
    mut command: Command,
    lookup: &impl SpecLookup,
    stack: &mut Vec<String>,
    missing: MissingLoadSpecPolicy,
) -> Result<Command, LoadSpecError> {
    if let Some(target_name) = command.load_spec.clone() {
        command = compose_load_spec(command, &target_name, lookup, stack, missing)?;
    }

    let mut resolved_subcommands = Vec::with_capacity(command.subcommands.len());
    for subcommand in command.subcommands {
        resolved_subcommands.push(resolve_tree(subcommand, lookup, stack, missing)?);
    }
    command.subcommands = resolved_subcommands;
    Ok(command)
}

fn compose_load_spec(
    wrapper: Command,
    target_name: &str,
    lookup: &impl SpecLookup,
    stack: &mut Vec<String>,
    missing: MissingLoadSpecPolicy,
) -> Result<Command, LoadSpecError> {
    let from = command_label(&wrapper);
    if stack.iter().any(|name| name == target_name) {
        let mut cycle = stack.clone();
        cycle.push(target_name.to_owned());
        return match missing {
            MissingLoadSpecPolicy::Error => Err(LoadSpecError::Cycle { stack: cycle }),
            MissingLoadSpecPolicy::Skip => {
                let mut skipped = wrapper;
                skipped.load_spec = None;
                Ok(skipped)
            }
        };
    }

    let Some(target) = lookup.get(target_name) else {
        return match missing {
            MissingLoadSpecPolicy::Error => Err(LoadSpecError::Missing {
                from,
                target: target_name.to_owned(),
            }),
            MissingLoadSpecPolicy::Skip => {
                let mut skipped = wrapper;
                skipped.load_spec = None;
                Ok(skipped)
            }
        };
    };

    stack.push(target_name.to_owned());
    let resolved_target = resolve_tree(target, lookup, stack, missing);
    stack.pop();
    let resolved_target = resolved_target?;
    Ok(compose_wrapper(wrapper, resolved_target))
}

fn compose_wrapper(mut wrapper: Command, mut target: Command) -> Command {
    wrapper.load_spec = None;
    if wrapper.description.is_none() {
        wrapper.description = target.description.take();
    }
    if wrapper.args.is_empty() {
        wrapper.args = target.args;
    }
    if wrapper.options.is_empty() {
        wrapper.options = target.options;
    } else {
        wrapper.options.extend(target.options);
    }
    if wrapper.subcommands.is_empty() {
        wrapper.subcommands = target.subcommands;
    } else {
        wrapper.subcommands.extend(target.subcommands);
    }
    wrapper
}

/// Walk a command tree and record every `loadSpec` that is missing or cyclic
/// under [`MissingLoadSpecPolicy::Error`]. Used by tests and corpus scans.
pub fn collect_load_spec_issues(command: Command, lookup: &impl SpecLookup) -> Vec<LoadSpecError> {
    let mut issues = Vec::new();
    collect_issues(
        command,
        lookup,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut issues,
    );
    issues
}

fn collect_issues(
    command: Command,
    lookup: &impl SpecLookup,
    stack: &mut Vec<String>,
    seen_missing: &mut HashSet<(String, String)>,
    issues: &mut Vec<LoadSpecError>,
) {
    if let Some(target_name) = command.load_spec.clone() {
        match compose_load_spec(
            command.clone(),
            &target_name,
            lookup,
            stack,
            MissingLoadSpecPolicy::Error,
        ) {
            Ok(composed) => {
                for subcommand in composed.subcommands {
                    collect_issues(subcommand, lookup, stack, seen_missing, issues);
                }
                return;
            }
            Err(err) => match &err {
                LoadSpecError::Missing { from, target } => {
                    if seen_missing.insert((from.clone(), target.clone())) {
                        issues.push(err);
                    }
                }
                LoadSpecError::Cycle { .. } => issues.push(err),
            },
        }
    }

    for subcommand in command.subcommands {
        collect_issues(subcommand, lookup, stack, seen_missing, issues);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fig_types::Command;
    use std::collections::HashMap;

    fn cmd(name: &str) -> Command {
        Command {
            name: vec![name.to_owned()],
            description: Some(format!("{name} spec")),
            ..Command::default()
        }
    }

    fn with_load_spec(mut command: Command, target: &str) -> Command {
        command.load_spec = Some(target.to_owned());
        command
    }

    fn lookup(specs: HashMap<&'static str, Command>) -> impl SpecLookup {
        move |name: &str| specs.get(name).cloned()
    }

    #[test]
    fn direct_composition_keeps_wrapper_metadata() {
        let mut flutter = cmd("flutter");
        flutter.subcommands = vec![cmd("analyze"), cmd("build")];

        let mut wrapper = cmd("flutter");
        wrapper.description = Some("Proxies Flutter commands".to_owned());
        wrapper = with_load_spec(wrapper, "flutter");

        let specs = HashMap::from([("flutter", flutter)]);
        let resolved =
            resolve_load_specs(wrapper, &lookup(specs), MissingLoadSpecPolicy::Error).unwrap();

        assert_eq!(resolved.name, vec!["flutter".to_owned()]);
        assert_eq!(
            resolved.description.as_deref(),
            Some("Proxies Flutter commands")
        );
        assert_eq!(
            resolved
                .subcommands
                .iter()
                .map(|c| c.name[0].as_str())
                .collect::<Vec<_>>(),
            vec!["analyze", "build"]
        );
        assert!(resolved.load_spec.is_none());
    }

    #[test]
    fn nested_composition_follows_each_reference() {
        let mut leaf = cmd("leaf");
        leaf.subcommands = vec![cmd("deep")];

        let mut mid = cmd("mid");
        let mut mid_child = cmd("inner");
        mid_child = with_load_spec(mid_child, "leaf");
        mid.subcommands = vec![mid_child];

        let wrapper = with_load_spec(cmd("root"), "mid");
        let specs = HashMap::from([("mid", mid), ("leaf", leaf)]);
        let resolved =
            resolve_load_specs(wrapper, &lookup(specs), MissingLoadSpecPolicy::Error).unwrap();

        assert_eq!(resolved.subcommands[0].name, vec!["inner".to_owned()]);
        assert_eq!(
            resolved.subcommands[0].subcommands[0].name,
            vec!["deep".to_owned()]
        );
    }

    #[test]
    fn missing_target_errors_when_policy_is_error() {
        let wrapper = with_load_spec(cmd("fvm-flutter"), "missing-spec");
        let err = resolve_load_specs(
            wrapper,
            &lookup(HashMap::new()),
            MissingLoadSpecPolicy::Error,
        )
        .unwrap_err();
        assert_eq!(
            err,
            LoadSpecError::Missing {
                from: "fvm-flutter".to_owned(),
                target: "missing-spec".to_owned(),
            }
        );
    }

    #[test]
    fn missing_target_is_skipped_when_policy_is_skip() {
        let wrapper = with_load_spec(cmd("aws-foo"), "aws/foo");
        let resolved = resolve_load_specs(
            wrapper,
            &lookup(HashMap::new()),
            MissingLoadSpecPolicy::Skip,
        )
        .unwrap();
        assert!(resolved.subcommands.is_empty());
        assert!(resolved.load_spec.is_none());
    }

    #[test]
    fn cycle_is_detected() {
        let a = with_load_spec(cmd("a"), "b");
        let b = with_load_spec(cmd("b"), "a");
        let specs = HashMap::from([("a", a.clone()), ("b", b)]);
        let err = resolve_load_specs(a, &lookup(specs), MissingLoadSpecPolicy::Error).unwrap_err();
        match err {
            LoadSpecError::Cycle { stack } => {
                assert!(stack.contains(&"a".to_owned()));
                assert!(stack.contains(&"b".to_owned()));
            }
            other => panic!("expected cycle, got {other:?}"),
        }
    }

    #[test]
    fn wrapper_options_are_prepended_to_target_options() {
        use crate::fig_types::CommandOption;

        let mut target = cmd("target");
        target.options = vec![CommandOption {
            name: vec!["--from-target".to_owned()],
            ..CommandOption::default()
        }];

        let mut wrapper = with_load_spec(cmd("wrap"), "target");
        wrapper.options = vec![CommandOption {
            name: vec!["--from-wrapper".to_owned()],
            ..CommandOption::default()
        }];

        let specs = HashMap::from([("target", target)]);
        let resolved =
            resolve_load_specs(wrapper, &lookup(specs), MissingLoadSpecPolicy::Error).unwrap();
        assert_eq!(
            resolved
                .options
                .iter()
                .map(|o| o.name[0].as_str())
                .collect::<Vec<_>>(),
            vec!["--from-wrapper", "--from-target"]
        );
    }

    #[test]
    fn collect_issues_reports_each_missing_target() {
        let mut root = cmd("aws");
        root.subcommands = vec![
            with_load_spec(cmd("foo"), "aws/foo"),
            with_load_spec(cmd("bar"), "aws/bar"),
        ];
        let issues = collect_load_spec_issues(root, &lookup(HashMap::new()));
        assert_eq!(issues.len(), 2);
    }
}
