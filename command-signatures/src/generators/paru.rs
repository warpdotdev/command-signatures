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
mod tests {
    use super::parse_package_list;

    #[test]
    fn test_parses_aur_and_repo_packages() {
        let output = "btrfs-progs core\nyay-bin AUR\n";
        let results = parse_package_list(output);

        let suggestions: Vec<(&str, Option<&str>)> = results
            .suggestions
            .iter()
            .map(|suggestion| {
                (
                    suggestion.exact_string.as_str(),
                    suggestion.description.as_deref(),
                )
            })
            .collect();

        assert_eq!(
            suggestions,
            vec![("btrfs-progs", Some("core")), ("yay-bin", Some("AUR"))]
        );
    }

    #[test]
    fn test_handles_single_field_line_without_panicking() {
        // A line with no space-separated source shouldn't panic, and should still surface the
        // package name (just without a description).
        let output = "btrfs-progs\n";
        let results = parse_package_list(output);

        assert_eq!(results.suggestions.len(), 1);
        assert_eq!(results.suggestions[0].exact_string, "btrfs-progs");
        assert_eq!(results.suggestions[0].description, None);
    }

    #[test]
    fn test_skips_blank_lines_and_lines_with_no_name() {
        let output = "\n \nbtrfs-progs core\n";
        let results = parse_package_list(output);

        assert_eq!(results.suggestions.len(), 1);
        assert_eq!(results.suggestions[0].exact_string, "btrfs-progs");
    }

    #[test]
    fn test_empty_output() {
        assert!(parse_package_list("").suggestions.is_empty());
    }
}
