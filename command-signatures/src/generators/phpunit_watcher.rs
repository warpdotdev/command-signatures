use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("phpunit").add_generator(
        "tests",
        Generator::script("phpunit --list-tests", |output| {
            if output.starts_with("fatal:") {
                return GeneratorResults::default();
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
                .collect_unordered_results()
        }),
    )
}
