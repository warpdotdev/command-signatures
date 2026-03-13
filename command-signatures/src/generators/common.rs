use warp_completion_metadata::{CommandBuilder, Generator, GeneratorResultsCollector, Suggestion};

/// Returns a cross-platform generator that lists local user names.
///
/// Uses `getent passwd` on Linux, `dscl` on macOS, and falls back to `/etc/passwd`.
pub fn users_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "sh -c 'if command -v getent >/dev/null 2>&1; then getent passwd | cut -d: -f1; elif command -v dscl >/dev/null 2>&1; then dscl . -list /Users; else cut -d: -f1 /etc/passwd; fi'",
        ),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| {
                    !line.is_empty() && !line.starts_with('_') && !line.starts_with('#')
                })
                .map(|name| Suggestion::with_description(name.trim(), "User"))
                .collect_unordered_results()
        },
    )
}
