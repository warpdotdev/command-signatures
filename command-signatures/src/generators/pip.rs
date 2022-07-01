use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("pip").add_generator(
        "list_packages",
        Generator::new("php list", |output| {
            output
                .split('\n')
                .skip(2)
                .map(Suggestion::new)
                .collect::<Vec<_>>()
        }),
    )
}
