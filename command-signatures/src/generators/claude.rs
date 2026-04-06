use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("claude")
        .add_generator(
            "mcp_servers",
            Generator::script(
                CommandBuilder::single_command("claude mcp list 2>/dev/null"),
                |output| {
                    // `claude mcp list` outputs lines like:
                    //   test-server: echo test - ✗ Failed to connect
                    //   sentry: npx sentry-mcp - ✓ Connected
                    // The server name is the text before the first colon.
                    // When no servers are configured it prints a "No MCP servers" message.
                    if output.contains("No MCP servers") {
                        return GeneratorResults::default();
                    }

                    output
                        .lines()
                        .filter_map(|line| {
                            let name = line.split(':').next()?.trim();
                            if name.is_empty() || name.starts_with("Checking") || name.contains(' ')
                            {
                                return None;
                            }
                            Some(Suggestion::with_description(name, "MCP server"))
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "installed_plugins",
            Generator::script(
                CommandBuilder::single_command("claude plugin list 2>/dev/null"),
                |output| {
                    // `claude plugin list` outputs lines like:
                    //   my-plugin (user, enabled)
                    //   other-plugin (project, disabled)
                    // The plugin name is the first token on each line.
                    // When no plugins are installed it prints a "No plugins" message.
                    if output.contains("No plugins") {
                        return GeneratorResults::default();
                    }

                    output
                        .lines()
                        .filter_map(|line| {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                return None;
                            }
                            let name = trimmed.split_whitespace().next()?;
                            if name.is_empty() || name.starts_with('(') {
                                return None;
                            }
                            // Extract the parenthesized description if present.
                            let description = trimmed
                                .find('(')
                                .and_then(|start| {
                                    trimmed.find(')').map(|end| &trimmed[start + 1..end])
                                })
                                .unwrap_or("Plugin");
                            Some(Suggestion::with_description(name, description))
                        })
                        .collect_unordered_results()
                },
            ),
        )
}
