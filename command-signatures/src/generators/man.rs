use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("man").add_generator(
        "list_man_pages",
        Generator::command_from_tokens(
            |context| {
                let section_glob = match context.last() {
                    Some(maybe_section) if maybe_section.len() == 1 && *maybe_section >= "1" && *maybe_section <= "8" => maybe_section,
                    // On a high-level, this code is try to get man completions for type 1 (general commands) and 8 (system admin and daemons) packages.
                    // man -w gives the all directories that contain man pages. These directories are further broken into sub-directories categorized by the type number.
                    // For example, a general command will live in man1 package and a system command will live in man8.
                    // What this command is doing is to chain all the different directories that have man pages and run ls to get all the page names. The [] operator in ls
                    // gives all possible values to match on. For example, ls man[18] will run ls man1 and ls man8 and chain together the output
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
