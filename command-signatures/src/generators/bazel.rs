use regex::Regex;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Importance,
    Order, Priority, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("bazel").add_generator(
        "build_file",
        // returns filepaths and contents in the form below, note the "----" to indicate the filepath
        // ----.//lib/BUILD
        // load("@rules_cc//cc:defs.bzl", "cc_library")

        // cc_library(
        //     name = "hello-time",
        //     srcs = ["hello-time.cc"],
        //     hdrs = ["hello-time.h"],
        //     visibility = ["//main:__pkg__"],
        // )
        Generator::script(
            CommandBuilder::pipe(
                // Use -type f to ensure we only get files, and print with -print to ensure consistent output
                CommandBuilder::single_command(
                    r#"find ./ -type f \( -name BUILD.bazel -o -name BUILD \) -print"#,
                ),
                // Use xargs instead of while read to handle filenames more reliably
                CommandBuilder::single_command(r#"xargs -I {} sh -c 'echo "----{}"; cat "{}";'"#),
            ),
            |output| {
                println!("OUTPUT IS HERE");
                let mut targets = Vec::new();
                let mut current_path = String::new();
                for line in output.lines() {
                    let file_path = FILE_RE.captures(line);
                    let bazel_target = BAZEL_RE.captures(line);
                    if let Some(path) = file_path {
                        if let Some(path_match) = path.get(1) {
                            current_path = format!("{}:", path_match.as_str());
                        }
                    } else if let Some(bazel) = bazel_target {
                        if let Some(bazel_match) = bazel.get(1) {
                            let mut suggestion = Suggestion::with_description(
                                format!("{}{}", current_path.clone(), bazel_match.as_str()),
                                "Bazel target",
                            );
                            suggestion.priority = Priority::Global(Importance::More(Order(80)));
                            targets.push(suggestion);
                        }
                    }
                }
                targets.into_iter().collect_unordered_results()
            },
        ),
    )
}

lazy_static! {
    static ref FILE_RE: Regex = Regex::new(r"----.(.*)/BUILD(?:\.bazel)?").unwrap();
    static ref BAZEL_RE: Regex = Regex::new(r#"name = "(.*)""#).unwrap();
}
