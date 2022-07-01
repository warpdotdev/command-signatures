use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("tar").add_generator(
        "list_tar_files",
        Generator::new("ls -1 | grep '.tar'", |output| {
            output
                .trim()
                .split('\n')
                .map(Suggestion::new)
                .collect::<Vec<_>>()
        }),
    )
}
