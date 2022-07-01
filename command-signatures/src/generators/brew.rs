use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("brew").add_generator(
        "services",
        Generator::new(
            "brew services list | sed -e 's/ .*//' | tail -n +2",
            |output | {
                output
                    .trim()
                    .split('\n')
                    .filter_map(|line| {
                        if line.contains("unbound") {
                            None
                        } else {
                            Some(Suggestion::new(line))
                        }
                    })
                    .collect::<Vec<_>>()
            },
        ),
    ).add_generator(
        "formulae_generator",
        Generator::new("brew list -1", |output| {
            output
                .trim()
                .split('\n')
                .filter_map(|line| {
                    if line.contains('=') {
                        return None;
                    }

                    Some(Suggestion::with_description(line, "Installed formula"))
                })
                .collect::<Vec<_>>()
        }),
    )
        .add_generator(
        "brew_info_generator",
        Generator::new(
            "HBPATH=$(brew --repository); ls -1 $HBPATH/Library/Taps/homebrew/homebrew-core/Formula $HBPATH/Library/Taps/homebrew/homebrew-cask/Casks",
            |output| {
                output
                    .trim()
                    .split('\n')
                    .map(|line| {
                        Suggestion::with_description(line.replace(".rb", ""), "formula")

                    })
                    .collect::<Vec<_>>()
            }
        ),
    )
        .add_generator(
            "uninstall_cask",
            Generator::new(
                "brew list -1 --cask",
                |output| {
                    output
                        .trim()
                        .split('\n')
                        .map(|formula| {
                            Suggestion::with_description(formula, "Installed formula")
                        })
                        .collect::<Vec<_>>()
                }
            ),
        )
}
