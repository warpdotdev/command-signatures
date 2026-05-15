use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use super::common;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pkill")
        .add_generator("user_name", common::users_generator())
        .add_generator(
            "process_name",
            Generator::script(
                CommandBuilder::pipe(
                    CommandBuilder::single_command("ps -A -o comm"),
                    CommandBuilder::single_command("sort -u"),
                ),
                process_names,
            ),
        )
}

fn process_names(output: &str) -> warp_completion_metadata::GeneratorResults {
    output
        .trim()
        .lines()
        .filter_map(|path| {
            let name = path.rsplit('/').next()?;
            if !name.is_empty() && name != "COMMAND" {
                Some(Suggestion::with_description(name, path))
            } else {
                None
            }
        })
        .collect_unordered_results()
}

#[cfg(test)]
mod tests {
    use super::process_names;

    #[test]
    fn test_process_names_handles_paths_and_plain_names() {
        let suggestions = process_names("COMMAND\n/usr/bin/python3\nbash\n/System/Library/foo\n");
        let names = suggestions
            .suggestions
            .into_iter()
            .map(|suggestion| suggestion.exact_string)
            .collect::<Vec<_>>();

        assert!(names.contains(&"python3".into()));
        assert!(names.contains(&"bash".into()));
        assert!(names.contains(&"foo".into()));
        assert!(!names.contains(&"COMMAND".into()));
    }
}
