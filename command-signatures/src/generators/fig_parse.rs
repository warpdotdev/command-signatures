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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_skips_blank_and_trims() {
        let results = lines("  alpha  \n\nbeta\n");
        let names: Vec<_> = results
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert_eq!(names, vec!["alpha", "beta"]);
        assert!(!results.is_ordered);
    }

    #[test]
    fn lines_empty_output_is_empty() {
        let results = lines("   \n\n");
        assert!(results.suggestions.is_empty());
    }
}
