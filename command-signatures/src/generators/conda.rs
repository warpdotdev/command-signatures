use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("conda")
        .add_generator(
            "get_installed_packages",
            Generator::script(CommandBuilder::single_command("conda list"), |output| {
                output
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .filter_map(|line| line.split_whitespace().next().map(Suggestion::new))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "get_conda_environments",
            Generator::script(CommandBuilder::single_command("conva env list"), |output| {
                output
                    .trim()
                    .split('\n')
                    .skip(2)
                    .filter_map(|line| line.split(' ').next().map(Suggestion::new))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "get_conda_configs",
            Generator::script(
                CommandBuilder::single_command("conda config --show"),
                |output| {
                    output
                        .trim()
                        .split('\n')
                        .skip(2)
                        .filter_map(|line| line.split(':').next().map(Suggestion::new))
                        .collect_unordered_results()
                },
            ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_installed_packages_post_process() {
        let gen = generator();
        let generator = gen
            .generators()
            .iter()
            .find(|(name, _)| name.0 == "get_installed_packages")
            .unwrap()
            .1;

        let output = "# packages in environment at /opt/conda:\n#\n# Name                    Version                   Build  Channel\n\nnumpy                     1.26.4          py311h64a7726_0    conda-forge\npip                       24.0               pyhd8ed1ab_0    conda-forge\n  python                    3.11.9          h955ad1f_0\n";
        let results = generator.on_complete(output);
        let names: Vec<_> = results
            .suggestions
            .iter()
            .map(|s| s.exact_string.as_str())
            .collect();
        assert_eq!(names, vec!["numpy", "pip", "python"]);
    }
}
