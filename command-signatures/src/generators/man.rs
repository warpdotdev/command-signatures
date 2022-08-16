use std::collections::HashSet;

use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("man").add_generator(
        "list_man_pages",
        Generator::command_from_tokens(
            |context| {
                let section_glob = match context.get(context.len() - 2) {
                    Some(maybe_section) if SECTION_NAMES.contains(maybe_section) => maybe_section,
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

lazy_static! {
    static ref SECTION_NAMES: HashSet<&'static str> =
        HashSet::from(["1", "2", "3", "4", "5", "6", "7", "8"]);
}
