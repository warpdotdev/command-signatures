use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("defaults").add_generator(
        "domain_generator",
        Generator::new("defaults domain", |output| {
            output
                .trim()
                .split(',')
                .map(|line| Suggestion::new(line.trim()))
                .collect::<Vec<_>>()
        }),
    )
}
