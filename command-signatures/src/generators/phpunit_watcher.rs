use warp_completion_metadata::{CommandGenerators, Generator, Suggestion};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("phpunit").add_generator(
        "tests",
        Generator::new("phpunit --list-tests", |output| {
            if output.starts_with("fatal:") {
                return vec![];
            }

            output
                .split('\n')
                .filter_map(|line| {
                    if let Some(index) = line.find("::") {
                        if index > 0 && (index + 2 < line.len()) {
                            return Some(Suggestion::new(&line[index + 2..]));
                        }
                    }
                    None
                })
                .collect::<Vec<_>>()
        }),
    )
}
