use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
    TemplateFilter,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("dotnet")
        .add_generator(
            "global_tools",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr("dotnet tool list --global"),
                |output| {
                    output
                        .lines()
                        .skip(2)
                        .flat_map(|line| {
                            let columns = line.split_whitespace().collect::<Vec<_>>();
                            columns
                                .get(2..)
                                .unwrap_or_default()
                                .join(" ")
                                .split(',')
                                .map(str::trim)
                                .filter(|command| !command.is_empty())
                                .map(|command| {
                                    let name = command.strip_prefix("dotnet-").unwrap_or(command);
                                    Suggestion::with_description(name, command)
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_filter(
            "filter-dll-files",
            TemplateFilter(|suggestion, path_type| {
                (path_type.is_folder() || suggestion.exact_string.ends_with(".dll"))
                    .then_some(suggestion)
            }),
        )
}
