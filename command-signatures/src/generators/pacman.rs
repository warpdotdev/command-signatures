/// Shared generator logic for `pacman` and the pacman-compatible AUR helpers `yay` and `paru`.
/// `yay` and `paru` are drop-in `pacman` wrappers that add AUR support on top of the same
/// pacman database and CLI syntax, so their completions are modeled on pacman's and, where the
/// underlying data is identical (e.g. installed packages, local `.pkg.tar` files), share the
/// exact same generator implementation. See `super::yay` and `super::paru` for what differs
/// (namely, how the list of *available* packages, including AUR packages, is enumerated).
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Priority, Suggestion,
};

pub const LIST_PKG_TAR_FILES_COMMAND: &str = r#"find . -maxdepth 1 -type f -name '*.pkg.tar' -o -name '*.pkg.tar.zst' -o -name '*.pkg.tar.gz' -o -name '*.pkg.tar.xz'"#;

/// Parses a newline-separated list of package names into suggestions. Used both for installed
/// packages (`pacman -Q` piped through `awk '{print $1}'`) and for available packages
/// (`pacman -Slq`, `yay -Pc`, `paru -Pc`), since all of those commands emit one package name per
/// line.
pub fn list_packages(output: &str) -> GeneratorResults {
    let mut targets = Vec::new();
    for package_name in output.lines() {
        if !package_name.is_empty() {
            targets.push(Suggestion::with_description(
                package_name.to_string(),
                "package",
            ));
        }
    }
    targets.into_iter().collect_unordered_results()
}

/// Parses the `find` output listing `.pkg.tar*` files in the current directory. These are
/// prioritized over installed/available packages, since a literal built package file is a
/// stronger completion signal than a bare package name.
pub fn list_pkg_tar_files_in_cwd(output: &str) -> GeneratorResults {
    let mut targets = Vec::new();
    for file in output.lines() {
        if !file.is_empty() {
            targets.push(
                Suggestion::with_description(file.to_string(), ".pkg.tar file")
                    .with_priority(Priority::most_important()),
            )
        }
    }
    targets.into_iter().collect_unordered_results()
}

/// Returns a generator that lists installed packages via `pacman -Q`. This is shared across
/// `pacman`, `yay`, and `paru`, since installed packages always live in the same pacman
/// database regardless of which frontend originally installed them; there is no need to shell
/// out to `yay`/`paru` themselves for this.
pub fn list_installed_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::pipe(
            CommandBuilder::single_command("pacman -Q"),
            CommandBuilder::single_command("awk '{print $1}'"),
        ),
        list_packages,
    )
}

/// Returns a generator that lists `.pkg.tar*` files in the current directory, for `-U`/`--upgrade`.
/// Shared across `pacman`, `yay`, and `paru`, since none of them change how local package files
/// are named or discovered.
pub fn list_pkg_tar_files_in_cwd_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(LIST_PKG_TAR_FILES_COMMAND),
        list_pkg_tar_files_in_cwd,
    )
}

pub fn pacman_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pacman")
        .add_generator(
            "list_installed_packages",
            list_installed_packages_generator(),
        )
        .add_generator(
            "list_all_packages",
            Generator::script(CommandBuilder::single_command("pacman -Slq"), list_packages),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            list_pkg_tar_files_in_cwd_generator(),
        )
}
