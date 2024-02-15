/// Used for debian-based package managers like apt-get, aptitude, etc.
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Priority,
    Suggestion,
};

const LIST_ALL_PACKAGES_NAME: &str = "list_all_packages";
const LIST_ALL_PACKAGES_COMMAND: &str = r#"dpkg-query --show --showformat '${Package}\n'"#;

const LIST_ALL_DEB_FILES_NAME: &str = "list_all_deb_files_in_cwd";
const LIST_ALL_DEB_FILES_COMMAND: &str = r#"find . -maxdepth 1 -type f -name '*.deb'"#;

pub fn list_all_packages(output: &str) -> GeneratorResults {
    let mut targets = Vec::new();
    for package_name in output.lines() {
        targets.push(Suggestion::with_description(
            package_name.to_string(),
            "package",
        ));
    }
    targets.into_iter().collect_unordered_results()
}

pub fn list_all_deb_files_in_cwd(output: &str) -> GeneratorResults {
    let mut targets = Vec::new();
    for file in output.lines() {
        targets.push(
            // We should prioritize .deb files over the already installed packages.
            Suggestion::with_description(file.to_string(), ".deb file")
                .with_priority(Priority::most_important()),
        )
    }
    targets.into_iter().collect_unordered_results()
}

pub fn apt_get_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("apt-get")
        .add_generator(
            LIST_ALL_PACKAGES_NAME,
            Generator::script(LIST_ALL_PACKAGES_COMMAND, list_all_packages),
        )
        .add_generator(
            LIST_ALL_DEB_FILES_NAME,
            Generator::script(LIST_ALL_DEB_FILES_COMMAND, list_all_deb_files_in_cwd),
        )
}

pub fn aptitude_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("aptitude")
        .add_generator(
            LIST_ALL_PACKAGES_NAME,
            Generator::script(LIST_ALL_PACKAGES_COMMAND, list_all_packages),
        )
        .add_generator(
            LIST_ALL_DEB_FILES_NAME,
            Generator::script(LIST_ALL_DEB_FILES_COMMAND, list_all_deb_files_in_cwd),
        )
}
