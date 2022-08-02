use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("bazel").add_generator(
        "build_file",
        Generator::new("FILES=( $(find ./ -name BUILD) ); for f in $FILES; do echo \"----$f\"; \\cat \"$f\"; done", |output| {
            let mut targets = Vec::new();
            let mut current_path = String::new();
            for line in output.lines() {
                let file_path = FILE_RE.captures(line);
                let bazel_target = BAZEL_RE.captures(line);
                if let Some(path) = file_path {
                    current_path = format!("{}:", &path[1]);
                } else if let Some(bazel) = bazel_target {
                    targets.push(Suggestion::with_description(current_path.clone() + &bazel[1], "Bazel target"))
                }
            }
            targets.into_iter().collect_unordered_results()
        }),
    )
}

lazy_static! {
    static ref FILE_RE: Regex = Regex::new(r"----.(.*)/BUILD").unwrap();
    static ref BAZEL_RE: Regex = Regex::new(r#"name = "(.*)""#).unwrap();
}
