use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub(super) fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ros2")
        .add_generator("packages", packages_generator())
        .add_generator("executables", executables_generator())
        .add_generator("topics", topics_generator())
        .add_generator("nodes", nodes_generator())
        .add_generator("services", services_generator())
        .add_generator("actions", actions_generator())
        .add_generator("interfaces", interfaces_generator())
}

fn packages_generator() -> Generator {
    Generator::script(CommandBuilder::single_command("ros2 pkg list"), |output| {
        output
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Suggestion::with_description(line.trim(), "ROS 2 package"))
            .collect_unordered_results()
    })
}

fn executables_generator() -> Generator {
    Generator::command_from_tokens(
        |tokens, has_trailing_whitespace, _env_vars| {
            let package = if has_trailing_whitespace {
                tokens.last().copied()
            } else {
                tokens.get(tokens.len().saturating_sub(2)).copied()
            };

            match package {
                Some(pkg) if !pkg.is_empty() => {
                    CommandBuilder::single_command(format!("ros2 pkg executables {pkg}"))
                }
                _ => CommandBuilder::single_command(""),
            }
        },
        |output| {
            output
                .trim()
                .lines()
                .filter_map(|line| {
                    // Output format is "<package_name> <executable_name>"
                    let mut parts = line.split_whitespace();
                    let _package = parts.next()?;
                    let executable = parts.next()?;
                    Some(Suggestion::new(executable))
                })
                .collect_unordered_results()
        },
    )
}

fn topics_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("ros2 topic list"),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        },
    )
}

fn nodes_generator() -> Generator {
    Generator::script(CommandBuilder::single_command("ros2 node list"), |output| {
        output
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Suggestion::new(line.trim()))
            .collect_unordered_results()
    })
}

fn services_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("ros2 service list"),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        },
    )
}

fn actions_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("ros2 action list"),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        },
    )
}

fn interfaces_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("ros2 interface list"),
        |output| {
            output
                .trim()
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    // Skip section headers like "Messages:", "Services:", "Actions:"
                    if trimmed.is_empty() || trimmed.ends_with(':') {
                        return None;
                    }
                    Some(Suggestion::new(trimmed))
                })
                .collect_unordered_results()
        },
    )
}
