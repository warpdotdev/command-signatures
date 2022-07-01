use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("man").add_generator(
        "list_man_pages",
        Generator::new(
            "ls -1 $(man -w | sed 's#:#/man1 #g') | cut -f 1 -d . | sort | uniq",
            |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        (!line.is_empty() && !line.starts_with('/'))
                            .then(|| Suggestion::with_description(line.trim(), "Man page"))
                    })
                    .collect::<Vec<_>>()
            },
        ),
    )
}
