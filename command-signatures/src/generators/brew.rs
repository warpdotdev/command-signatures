use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("brew")
        .add_generator(
            "services",
            Generator::new(
                "brew services list | sed -e 's/ .*//' | tail -n +2",
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter_map(|line| {
                            if line.contains("unbound") {
                                None
                            } else {
                                Some(Suggestion::new(line))
                            }
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "formulae_generator",
            Generator::new("brew list -1", |output| {
                output
                    .trim()
                    .lines()
                    .filter_map(|line| {
                        if line.contains('=') {
                            return None;
                        }

                        Some(Suggestion::with_description(line, "Installed formula"))
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "brew_info_generator",
            Generator::new(
                "HBPATH=$(brew --repository); ls -1 $HBPATH/Library/Taps/homebrew/h\
            omebrew-core/Formula $HBPATH/Library/Taps/homebrew/homebrew-cask/Casks",
                |output| {
                    output
                        .trim()
                        .lines()
                        .map(|line| {
                            Suggestion::with_description(line.replace(".rb", ""), "formula")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "uninstall_cask",
            Generator::new("brew list -1 --cask", |output| {
                output
                    .trim()
                    .lines()
                    .map(|formula| Suggestion::with_description(formula, "Installed formula"))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "outdated_formula_generator",
            Generator::new("brew outdated -q", post_process),
        )
        .add_generator(
            "repositories_generator",
            Generator::new("brew tap", post_process),
        )
}

fn post_process(output: &str) -> GeneratorResults {
    output
        .trim()
        .lines()
        .map(Suggestion::new)
        .collect_unordered_results()
}
