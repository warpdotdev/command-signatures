use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Priority, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("apt-get")
        .add_generator(
            "list_all_packages",
            Generator::script(
                r#"dpkg-query --show --showformat '${Package}\n'"#,
                |output| {
                    let mut targets = Vec::new();
                    for package_name in output.lines() {
                        targets.push(Suggestion::with_description(
                            package_name.to_string(),
                            "package",
                        ));
                    }
                    targets.into_iter().collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "list_all_deb_files_in_cwd",
            Generator::script(r#"find . -maxdepth 1 -type f -name '*.deb'"#, |output| {
                let mut targets = Vec::new();
                for file in output.lines() {
                    targets.push(
                        // We should prioritize .deb files over the already installed packages.
                        Suggestion::with_description(file.to_string(), ".deb file")
                            .with_priority(Priority::most_important()),
                    )
                }
                targets.into_iter().collect_unordered_results()
            }),
        )
}
