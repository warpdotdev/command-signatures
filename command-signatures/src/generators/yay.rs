use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use super::common::{pacman_installed_packages_generator, pacman_pkg_tar_files_in_cwd_generator};

/// Parses the tab-separated `name\tsource` lines produced by `yay -P/--show -c/--complete`,
/// where `source` is either `AUR` or the sync repository the package belongs to (e.g. `core`,
/// `extra`). See `createAURList`/`createRepoList` in yay's `pkg/completion/completion.go`, which
/// write `pkgName+"\tAUR\n"` and `pkg.Name()+"\t"+pkg.DB().Name()+"\n"` respectively.
fn parse_package_list(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let mut fields = line.splitn(2, '\t');
            let name = fields.next()?.trim();
            if name.is_empty() {
                return None;
            }
            Some(
                match fields
                    .next()
                    .map(str::trim)
                    .filter(|source| !source.is_empty())
                {
                    Some(source) => Suggestion::with_description(name, source),
                    None => Suggestion::new(name),
                },
            )
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("yay")
        .add_generator(
            "list_installed_packages",
            pacman_installed_packages_generator(),
        )
        .add_generator(
            "list_all_packages",
            Generator::script(
                CommandBuilder::single_command("yay -Pc"),
                parse_package_list,
            ),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            pacman_pkg_tar_files_in_cwd_generator(),
        )
}

#[cfg(test)]
#[path = "yay_tests.rs"]
mod tests;
