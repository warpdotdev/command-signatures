use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Priority, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("dnf")
        .add_generator(
            "list_installed_packages",
            Generator::script(r#"dnf list --installed | awk '{print $1}'"#, |output| {
                let mut targets = Vec::new();
                for package_name in output.lines() {
                    targets.push(Suggestion::with_description(
                        package_name.to_string(),
                        "package",
                    ));
                }
                targets.into_iter().collect_unordered_results()
            }),
        )
        .add_generator(
            "list_rpm_files_in_cwd",
            Generator::script(r#"find . -maxdepth 1 -type f -name '*.rpm'"#, |output| {
                // We should prioritize .rpm files over the already installed packages.
                let mut targets = Vec::new();
                for file in output.lines() {
                    if !file.is_empty() {
                        targets.push(
                            Suggestion::with_description(file.to_string(), ".rpm file")
                                .with_priority(Priority::most_important()),
                        )
                    }
                }
                targets.into_iter().collect_unordered_results()
            }),
        )
}
