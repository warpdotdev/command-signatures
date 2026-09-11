use crate::fig_types::Command;
use std::collections::HashSet;
use std::fmt;

/// Failure while validating a static `loadSpec` reference graph.
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

impl PartialOrd for LoadSpecError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LoadSpecError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl LoadSpecError {
    fn sort_key(&self) -> (u8, String, String) {
        match self {
            LoadSpecError::Missing { from, target } => (0, from.clone(), target.clone()),
            LoadSpecError::Cycle { stack } => (1, stack.join(" -> "), String::new()),
        }
    }
}

/// Looks up a command spec by the `loadSpec` target name (for example `"flutter"`
/// or `"gcloud/ai-platform"`).
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

fn command_label(command: &Command) -> String {
    command
        .name
        .first()
        .cloned()
        .unwrap_or_else(|| "<unnamed>".to_owned())
}

/// Walk a command tree and record every `loadSpec` that is missing or cyclic.
/// Follows exact and slash-path targets without composing wrapper and target trees.
pub fn collect_load_spec_issues(command: &Command, lookup: &impl SpecLookup) -> Vec<LoadSpecError> {
    let mut issues = Vec::new();
    collect_issues(
        command,
        lookup,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut issues,
    );
    issues.sort();
    issues
}

fn collect_issues(
    command: &Command,
    lookup: &impl SpecLookup,
    stack: &mut Vec<String>,
    seen_missing: &mut HashSet<(String, String)>,
    issues: &mut Vec<LoadSpecError>,
) {
    if let Some(target_name) = command.load_spec.as_deref() {
        validate_reference(command, target_name, lookup, stack, seen_missing, issues);
    }

    for subcommand in &command.subcommands {
        collect_issues(subcommand, lookup, stack, seen_missing, issues);
    }
}

fn validate_reference(
    from: &Command,
    target_name: &str,
    lookup: &impl SpecLookup,
    stack: &mut Vec<String>,
    seen_missing: &mut HashSet<(String, String)>,
    issues: &mut Vec<LoadSpecError>,
) {
    if stack.iter().any(|name| name == target_name) {
        let mut cycle = stack.clone();
        cycle.push(target_name.to_owned());
        issues.push(LoadSpecError::Cycle { stack: cycle });
        return;
    }

    let Some(target) = lookup.get(target_name) else {
        let from_label = command_label(from);
        if seen_missing.insert((from_label.clone(), target_name.to_owned())) {
            issues.push(LoadSpecError::Missing {
                from: from_label,
                target: target_name.to_owned(),
            });
        }
        return;
    };

    stack.push(target_name.to_owned());
    collect_issues(&target, lookup, stack, seen_missing, issues);
    stack.pop();
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
    fn exact_target_is_present_and_not_composed() {
        let flutter = {
            let mut flutter = cmd("flutter");
            flutter.subcommands = vec![cmd("analyze")];
            flutter
        };
        let wrapper = with_load_spec(cmd("flutter"), "flutter");
        let specs = HashMap::from([("flutter", flutter)]);
        let issues = collect_load_spec_issues(&wrapper, &lookup(specs));
        assert!(issues.is_empty(), "{issues:?}");
        assert_eq!(wrapper.load_spec.as_deref(), Some("flutter"));
        assert!(wrapper.subcommands.is_empty());
    }

    #[test]
    fn slash_path_target_is_looked_up_by_full_name() {
        let mut platform = cmd("ai-platform");
        platform.subcommands = vec![cmd("jobs")];
        let wrapper = with_load_spec(cmd("ai-platform"), "gcloud/ai-platform");
        let specs = HashMap::from([("gcloud/ai-platform", platform)]);
        let issues = collect_load_spec_issues(&wrapper, &lookup(specs));
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn nested_references_are_followed_without_composing() {
        let mut leaf = cmd("leaf");
        leaf.subcommands = vec![cmd("deep")];

        let mut mid = cmd("mid");
        let mut mid_child = cmd("inner");
        mid_child = with_load_spec(mid_child, "leaf");
        mid.subcommands = vec![mid_child];

        let wrapper = with_load_spec(cmd("root"), "mid");
        let specs = HashMap::from([("mid", mid.clone()), ("leaf", leaf)]);
        let issues = collect_load_spec_issues(&wrapper, &lookup(specs));
        assert!(issues.is_empty(), "{issues:?}");
        assert_eq!(wrapper.load_spec.as_deref(), Some("mid"));
        assert!(wrapper.subcommands.is_empty());
        assert_eq!(mid.subcommands[0].load_spec.as_deref(), Some("leaf"));
        assert!(mid.subcommands[0].subcommands.is_empty());
    }

    #[test]
    fn missing_target_is_reported() {
        let wrapper = with_load_spec(cmd("fvm-flutter"), "missing-spec");
        let issues = collect_load_spec_issues(&wrapper, &lookup(HashMap::new()));
        assert_eq!(
            issues,
            vec![LoadSpecError::Missing {
                from: "fvm-flutter".to_owned(),
                target: "missing-spec".to_owned(),
            }]
        );
    }

    #[test]
    fn cycle_is_detected() {
        let a = with_load_spec(cmd("a"), "b");
        let b = with_load_spec(cmd("b"), "a");
        let specs = HashMap::from([("a", a.clone()), ("b", b)]);
        let issues = collect_load_spec_issues(&a, &lookup(specs));
        assert!(
            issues
                .iter()
                .any(|issue| matches!(issue, LoadSpecError::Cycle { .. })),
            "{issues:?}"
        );
    }

    #[test]
    fn collect_issues_reports_each_missing_target_deterministically() {
        let mut root = cmd("aws");
        root.subcommands = vec![
            with_load_spec(cmd("bar"), "aws/bar"),
            with_load_spec(cmd("foo"), "aws/foo"),
        ];
        let issues = collect_load_spec_issues(&root, &lookup(HashMap::new()));
        assert_eq!(
            issues,
            vec![
                LoadSpecError::Missing {
                    from: "bar".to_owned(),
                    target: "aws/bar".to_owned(),
                },
                LoadSpecError::Missing {
                    from: "foo".to_owned(),
                    target: "aws/foo".to_owned(),
                },
            ]
        );
    }
}
