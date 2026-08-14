use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

use super::common::{pacman_installed_packages_generator, pacman_pkg_tar_files_in_cwd_generator};

/// Parses the space-separated `name source` lines produced by `paru -P/--show -c/--complete`,
/// where `source` is either `AUR` or the (sync or local pkgbuild) repository the package
/// belongs to (e.g. `core`, `extra`). See `repo_list`/`pkgbuild_list`/`aur_list` in paru's
/// `src/completion.rs`, which write `pkg.name()`, `b" "`, `db.name()`, `b"\n"` and
/// `line`, `b" AUR\n"` respectively.
fn parse_package_list(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter_map(|line| {
            let mut fields = line.splitn(2, ' ');
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
    CommandSignatureGenerators::new("paru")
        .add_generator(
            "list_installed_packages",
            pacman_installed_packages_generator(),
        )
        .add_generator(
            "list_all_packages",
            Generator::script(
                CommandBuilder::single_command("paru -Pc"),
                parse_package_list,
            ),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            pacman_pkg_tar_files_in_cwd_generator(),
        )
}

#[cfg(test)]
#[path = "paru_tests.rs"]
mod tests;
