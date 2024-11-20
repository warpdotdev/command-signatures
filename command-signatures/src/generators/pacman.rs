use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Priority,
    Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pacman")
        .add_generator(
            "list_installed_packages",
            Generator::script(CommandBuilder::pipe(CommandBuilder::single_command(r#"pacman -Q"#), CommandBuilder::single_command(r#"awk '{print $1}'"#)), |output| {
                let mut targets = Vec::new();
                for package_name in output.lines() {
                    targets.push(Suggestion::with_description(
                        package_name.to_string(),
                        "package",
                    ));
                }
                targets.into_iter().collect_unordered_results()
            }),
        ).add_generator(
            "list_all_packages",
            Generator::script(CommandBuilder::single_command("pacman -Slq"), |output| {
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
            "list_all_pkg_tar_files_in_cwd",
            Generator::script(
                CommandBuilder::single_command(r#"find . -maxdepth 1 -type f -name '*.pkg.tar' -o -name '*.pkg.tar.zst' -o -name '*.pkg.tar.gz' -o -name '*.pkg.tar.xz'"#),
                |output| {
                    // We should prioritize .pkg.tar files over the already installed packages.
                    let mut targets = Vec::new();
                    for file in output.lines() {
                        if !file.is_empty() {
                            targets.push(Suggestion::with_description(file.to_string(), ".pkg.tar file").with_priority(Priority::most_important()))
                        }
                    }
                    targets.into_iter().collect_unordered_results()
                },
            ),
        )
}
