use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("man").add_generator(
        "list_man_pages",
        Generator::command_from_tokens(
            |context| {
                let section_glob = match context.last() {
                    Some(maybe_section) if maybe_section.len() == 1 && *maybe_section >= "1" && *maybe_section <= "8" => maybe_section,
                    _ => "[18]"
                };
                format!("ls -1 $(man -w | sed 's#:#/man{} #g') 2>/dev/null | cut -f 1 -d . | sort | uniq", section_glob)
            },
            |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        (!line.is_empty() && !line.starts_with('/'))
                            .then(|| Suggestion::with_description(line.trim(), "Man page"))
                    })
                    .collect_ordered_results()
            },
        ),
    )
}
