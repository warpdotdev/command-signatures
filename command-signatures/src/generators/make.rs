use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("make").add_generator("list_targets", Generator::new("make -qp | awk -F':' '/^[a-zA-Z0-9][^$#\\/\\t=]*:([^=]|$)/ {split($1,A,/ /);for(i in A)print A[i]}' | sort -u", |output|  {
        output.split('\n').map(|line| {
            Suggestion::with_description(line.trim(), "Make target")
        }).collect_from_unordered_suggestions()
    }))
}
