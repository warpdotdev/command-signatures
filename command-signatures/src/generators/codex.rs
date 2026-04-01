use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use super::git;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("codex")
        .add_generator("local_branches", git::local_branches_generator())
        .add_generator("commits", git::commits_generator())
        .add_generator(
            "mcp_servers",
            Generator::script(
                CommandBuilder::single_command("codex mcp list 2>/dev/null"),
                |output| {
                    // `codex mcp list` outputs lines like:
                    //   my-server  stdio  npx -y @modelcontextprotocol/server-everything
                    // The first column is the server name. Skip the header-like
                    // "No MCP servers configured" message.
                    if output.contains("No MCP servers configured") {
                        return GeneratorResults::default();
                    }

                    output
                        .lines()
                        .filter_map(|line| {
                            let name = line.split_whitespace().next()?;
                            if name.is_empty() {
                                return None;
                            }
                            Some(Suggestion::with_description(name, "MCP server"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "feature_flags",
            Generator::script(
                CommandBuilder::single_command("codex features list 2>/dev/null"),
                |output| {
                    // `codex features list` outputs lines like:
                    //   shell_tool  stable  true
                    // The first column is the feature name.
                    output
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let name = parts.next()?;
                            if name.is_empty() {
                                return None;
                            }
                            let stage = parts.next().unwrap_or("");
                            let state = parts.next().unwrap_or("");
                            let description = format!("{} ({})", stage, state);
                            Some(Suggestion::with_description(name, description))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "cloud_tasks",
            Generator::script(
                CommandBuilder::single_command("codex cloud list 2>/dev/null"),
                |output| {
                    // `codex cloud list` outputs lines like:
                    //   TASK_ID                              STATUS     QUERY
                    //   0199a213-81c0-7800-8aa1-bbab2a035a53 completed  Fix the test
                    // The first column is the task ID (UUID). Skip error or empty
                    // output when not authenticated.
                    if output.contains("Not signed in")
                        || output.contains("Please run")
                        || output.trim().is_empty()
                    {
                        return GeneratorResults::default();
                    }

                    output
                        .lines()
                        .filter_map(|line| {
                            let mut parts = line.split_whitespace();
                            let task_id = parts.next()?;
                            if task_id.is_empty() || !task_id.contains('-') {
                                // Skip header rows or non-UUID lines.
                                return None;
                            }
                            let status = parts.next().unwrap_or("");
                            // Collect the remaining columns as the task description.
                            let query: String = parts.collect::<Vec<_>>().join(" ");
                            let description = if query.is_empty() {
                                status.to_owned()
                            } else {
                                format!("{} — {}", status, query)
                            };
                            Some(Suggestion::with_description(task_id, description))
                        })
                        .collect_ordered_results()
                },
            ),
        )
}
