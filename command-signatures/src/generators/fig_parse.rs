use warp_completion_metadata::{GeneratorResults, GeneratorResultsCollector, Suggestion};

/// One suggestion per non-empty trimmed line.
pub fn lines(output: &str) -> GeneratorResults {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Suggestion::new)
        .collect_unordered_results()
}
